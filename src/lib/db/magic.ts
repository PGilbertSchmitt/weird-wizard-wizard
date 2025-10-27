import {
  MagicSpellRecord,
  MagicTalentRecord,
  MagicTraditionRecord,
} from '../import-data/magic-import';
import { dbExecute, id, Tables } from './client';

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
