import {
  MagicSpellRecord,
  MagicTalentRecord,
  MagicTraditionRecord,
} from '../import-data/magic-import';
import { SpellItem, FullTradition, TraditionIndexItem, TalentItem } from '../types';
import { dbExecute, dbSelect, id, Tables } from './client';

import { MagicTalentCharge, MagicTalentRestoration, PathKind, PathKinds } from './enums';
import { getInfoTables, getOptionBlocks } from './tables-and-options';

/* INSERT queries */

export const createTradition = async (
  record: MagicTraditionRecord,
  infoTableId: number | null,
) => {
  return id(
    Tables.TRADITIONS,
    await dbExecute(
      `
INSERT INTO ${Tables.TRADITIONS} (
  name,
  blurb,
  description,
  special_info,
  info_table_id
) VALUES ($1, $2, $3, $4, $5)`,
      [
        record.name,
        record.blurb,
        record.description,
        record.special_info,
        infoTableId,
      ],
    ),
  );
};

export const createMagicTalent = async (
  record: MagicTalentRecord,
  traditionId: number,
  infoTableId: number | null,
  optionBlockId: number | null,
) => {
  return id(
    Tables.TALENTS,
    await dbExecute(
      `
INSERT INTO ${Tables.TALENTS} (
  name,
  description,
  magical,
  charges,
  restore,
  tradition_id,
  info_table_id,
  option_block_id
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`,
      [
        record.talent_name,
        record.description,
        true,
        record.charges,
        record.restore,
        traditionId,
        infoTableId,
        optionBlockId,
      ],
    ),
  );
};

export const createActivationTag = async (tag: string) => {
  return id(
    Tables.ACTIVATE_TAGS,
    await dbExecute(`INSERT INTO ${Tables.ACTIVATE_TAGS} (name) VALUES ($1)`, [
      tag,
    ]),
  );
};

export const createMagicSpell = async (
  record: MagicSpellRecord,
  traditionId: number,
  infoTableId: number | null,
  optionBlockId: number | null,
) => {
  return id(
    Tables.SPELLS,
    await dbExecute(
      `
INSERT INTO ${Tables.SPELLS} (
  name,
  description,
  path_kind,
  castings,
  duration,
  target,
  condition,
  ritual,
  tradition_id,
  info_table_id,
  option_block_id
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)`,
      [
        record.name,
        record.description,
        record.path_type,
        record.castings,
        record.duration,
        record.target,
        record.condition,
        record.ritual,
        traditionId,
        infoTableId,
        optionBlockId,
      ],
    ),
  );
};

/* SELECT queries */

interface SpellRow {
  id: number;
  name: string;
  description: string;
  pathKind: PathKind;
  castings: number;
  duration: string;
  target: string;
  condition: string | null;
  ritual: boolean;
  infoTableId: number | null;
  optionBlockId: number | null;
}

// Does not populate the info tables or option blocks
const getSpellsByTradition = async (traditionId: number) =>
  dbSelect<SpellRow>(
    `SELECT
      id,
      name,
      description,
      path_kind as pathKind,
      castings,
      duration,
      target,
      condition,
      ritual,
      info_table_id as infoTableId,
      option_block_id as optionBlockId
    FROM ${Tables.SPELLS}
    WHERE tradition_id=$1`,
    [traditionId],
  );

interface TalentRow {
  id: number;
  name: string;
  description: string;
  magical: boolean;
  charges: MagicTalentCharge;
  restore: MagicTalentRestoration;
  infoTableId: number | null;
  optionBlockId: number | null;
}

const getTalentsByTradition = async (traditionId: number) =>
  dbSelect<TalentRow>(
    `SELECT
      id,
      name,
      description,
      magical,
      charges,
      restore,
      info_table_id as infoTableId,
      option_block_id as optionBlockId
    FROM ${Tables.TALENTS} WHERE tradition_id=$1`,
    [traditionId],
  );

export const getTraditions = () =>
  dbSelect<TraditionIndexItem>(
    `SELECT id, name, blurb, description FROM ${Tables.TRADITIONS}`,
  );

interface TraditionRow {
  id: number;
  name: string;
  blurb: string;
  description: string;
  specialInfo: string | null;
  infoTableId: number | null;
}

export const getTradition = async (
  id: number,
): Promise<FullTradition | null> => {
  const [traditionInfo] = await dbSelect<TraditionRow>(
    `SELECT
      id,
      name,
      blurb,
      description,
      special_info as specialInfo,
      info_table_id as infoTableId
    FROM ${Tables.TRADITIONS} WHERE id=$1`,
    [id],
  );
  if (!traditionInfo) {
    return null;
  };

  const [spellRows, talentRows] = await Promise.all([
    getSpellsByTradition(id),
    getTalentsByTradition(id),
  ]);

  const optionBlockIds: number[] = [];
  const infoTableIds: number[] = [];

  if (traditionInfo.infoTableId) {
    infoTableIds.push(traditionInfo.infoTableId);
  }

  for (const spell of spellRows) {
    if (spell.optionBlockId) {
      optionBlockIds.push(spell.optionBlockId);
    }
    if (spell.infoTableId) {
      infoTableIds.push(spell.infoTableId);
    }
  }

  for (const talent of talentRows) {
    if (talent.optionBlockId) {
      optionBlockIds.push(talent.optionBlockId);
    }
    if (talent.infoTableId) {
      infoTableIds.push(talent.infoTableId);
    }
  }

  const [optionBlockMap, infoTableMap] = await Promise.all([
    getOptionBlocks(optionBlockIds),
    getInfoTables(infoTableIds),
  ]);

  const talents = talentRows.map((row: TalentRow): TalentItem => ({
    id: row.id,
    name: row.name,
    description: row.description,
    magical: row.magical,
    charges: row.charges,
    restore: row.restore,
    infoTable: (row.infoTableId && infoTableMap.get(row.infoTableId)) || null,
    optionBlock: (row.optionBlockId && optionBlockMap.get(row.optionBlockId)) || null,
  }));

  const spells = spellRows.map((row: SpellRow): SpellItem => ({
    id: row.id,
    name: row.name,
    description: row.description,
    pathKind: row.pathKind,
    castings: row.castings,
    duration: row.duration,
    target: row.target,
    condition: row.condition,
    ritual: row.ritual,
    infoTable: (row.infoTableId && infoTableMap.get(row.infoTableId)) || null,
    optionBlock: (row.optionBlockId && optionBlockMap.get(row.optionBlockId)) || null,
  }));

  const groupedSpells = spells.reduce((acc, spell) => {
    switch (spell.pathKind) {
      case PathKinds.NOVICE:
        acc.Novice.push(spell);
        break;
      case PathKinds.EXPERT:
        acc.Expert.push(spell);
        break;
      case PathKinds.MASTER:
        acc.Master.push(spell);
        break;
    }
    return acc;
  }, {
    [PathKinds.NOVICE]: [],
    [PathKinds.EXPERT]: [],
    [PathKinds.MASTER]: [],
  } as Record<PathKind, SpellItem[]>);

  return {
    id: traditionInfo.id,
    name: traditionInfo.name,
    blurb: traditionInfo.blurb,
    description: traditionInfo.description,
    specialInfo: traditionInfo.specialInfo,
    infoTable:
      (traditionInfo.infoTableId &&
        infoTableMap.get(traditionInfo.infoTableId)) ||
      null,
    talents,
    noviceSpells: groupedSpells.Novice,
    expertSpells: groupedSpells.Expert,
    masterSpells: groupedSpells.Master,
  };
};
