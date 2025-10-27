// import { CSVParseResults } from './import-utils';
import { ImportData } from '.';
import { err, ok, Result } from 'neverthrow';
import { flatten, isNil, sum, toPairs, uniq, values } from 'ramda';
import { createProfession, createProfessionCategory } from '../db/professions';
import {
  createImmunity,
  createLanguage,
  createSense,
  createSpeedTrait,
} from '../db/natural-attributes';
import { createAncestry } from '../db/ancestries';
import {
  createAncestryImmunity,
  createAncestryLanguage,
  createAncestrySense,
  createAncestrySpeedTrait,
  createLevelLanguage,
  createLevelSpeedTrait,
  createMagicTalentActivation,
} from '../db/junctions';
import { CLEAR_TABLES, dbExecute } from '../db/client';
import { createNonNovicePath, createNovicePath } from '../db/paths';
import { PathKinds, TableType } from '../db/enums';
import { createLevel } from '../db/levels';
import {
  createInfoTable,
  createInfoTableRows,
  createOptionBlock,
  createOptionBlockRows,
} from '../db/tables-and-options';
import { ImportError } from './import-error';
import {
  createActivationTag,
  createMagicSpell,
  createMagicTalent,
  createTradition,
} from '../db/magic';

type IdMap = Map<string, number>;

type DbResult = Result<number, ImportError>;

export const importProcess = async (
  data: ImportData,
  onProgress: (workingOn: string, percent: number) => void,
): Promise<DbResult> => {
  try {
    await destroyAll();

    const totalRecords = countRecords(data);
    let currentRecords = 0;
    let currentPercentage = -1;
    let currentWork = 'Profession';

    const trackProgress = (count = 1) => {
      currentRecords += count;
      const newPercent = Math.floor((currentRecords / totalRecords) * 100);
      if (newPercent > currentPercentage) {
        currentPercentage = newPercent;
        onProgress(currentWork, currentPercentage);
      }
    };

    // Initial progress update
    trackProgress(0);

    await Promise.all(
      onlyOks(data.professionCategories).map(async (record) => {
        await createProfessionCategory(record);
        trackProgress();
      }),
    );

    await Promise.all(
      onlyOks(data.professions).map(async (record) => {
        await createProfession(record);
        trackProgress();
      }),
    );

    currentWork = 'Ancestries';

    const languageMap: IdMap = new Map(
      await Promise.all(
        onlyOks(data.languages).map(async (record) => {
          const langId = await createLanguage(record);
          trackProgress();
          return [record.name, langId] as const;
        }),
      ),
    );

    const speedTraitMap: IdMap = new Map(
      await Promise.all(
        onlyOks(data.speedTraits).map(async (record) => {
          const speedTraitId = await createSpeedTrait(record);
          trackProgress();
          return [record.name, speedTraitId] as const;
        }),
      ),
    );

    const senseMap: IdMap = new Map(
      await Promise.all(
        onlyOks(data.senses).map(async (record) => {
          const senseId = await createSense(record);
          trackProgress();
          return [record.name, senseId] as const;
        }),
      ),
    );

    const ancestries = onlyOks(data.ancestries);
    const immunities = uniq(flatten(ancestries.map((a) => a.immunities)));
    const immunityMap: IdMap = new Map(
      await Promise.all(
        immunities.map(async (immunity) => {
          const immunityId = await createImmunity(immunity);
          return [immunity, immunityId] as const;
        }),
      ),
    );

    const ancestryMap: IdMap = new Map(
      await Promise.all(
        ancestries.map(async (ancestry) => {
          const ancestryId = await createAncestry(ancestry);
          const ancestryName = ancestry.ancestry;

          await Promise.all([
            ...ancestry.languages.map(
              associateAncestryLanguage(languageMap, ancestryId, ancestryName),
            ),
            ...ancestry.speed_traits.map(
              associateAncestrySpeedTraits(
                speedTraitMap,
                ancestryId,
                ancestryName,
              ),
            ),
            ...ancestry.senses.map(
              associateAncestrySenses(senseMap, ancestryId, ancestryName),
            ),
            ...ancestry.immunities.map(
              associateAncestryImmunity(immunityMap, ancestryId, ancestryName),
            ),
          ]);
          trackProgress();
          return [ancestryName, ancestryId] as const;
        }),
      ),
    );

    currentWork = 'Paths and Levels';

    const pathMap: IdMap = new Map(
      await Promise.all([
        ...onlyOks(data.novicePaths).map(async (novicePath) => {
          const ancestryId = ancestryMap.get(novicePath.name);
          const pathId = await createNovicePath(novicePath, ancestryId);
          trackProgress();
          return [novicePath.name, pathId] as const;
        }),
        ...onlyOks(data.expertPaths).map(async (expertPath) => {
          const pathId = await createNonNovicePath(
            expertPath,
            PathKinds.EXPERT,
          );
          trackProgress();
          return [expertPath.name, pathId] as const;
        }),
        ...onlyOks(data.masterPaths).map(async (masterPath) => {
          const pathId = await createNonNovicePath(
            masterPath,
            PathKinds.MASTER,
          );
          trackProgress();
          return [masterPath.name, pathId] as const;
        }),
      ]),
    );

    await Promise.all(
      onlyOks([
        ...data.noviceLevels,
        ...data.expertLevels,
        ...data.masterLevels,
      ]).map(async (level) => {
        const pathId = pathMap.get(level.path);
        if (!pathId) {
          throw new Error(
            `Failed to associate a level with a path named '${level.path}', no such path was found`,
          );
        }

        const levelId = await createLevel(level, pathId);
        const levelLabel = `${level.path}:${level.level}`;

        await Promise.all([
          ...level.languages.map(
            associateLevelLanguages(languageMap, levelId, levelLabel),
          ),
          ...level.speed_traits.map(
            associateLevelSpeedTraits(speedTraitMap, levelId, levelLabel),
          ),
        ]);
        trackProgress();
      }),
    );

    currentWork = 'Tables and Info Blocks';

    const optionBlocks = onlyOks(data.magicOptions).reduce(
      (acc: Record<string, string[]>, row) => {
        acc[row.options_id] ||= [];
        acc[row.options_id].push(row.description);
        return acc;
      },
      {},
    );
    const optionBlockMap: IdMap = new Map(
      await Promise.all(
        toPairs(optionBlocks).map(async ([label, opts]) => {
          const blockId = await createOptionBlock(label);
          await createOptionBlockRows(blockId, opts);
          trackProgress(opts.length);
          return [label, blockId] as const;
        }),
      ),
    );

    type TableRow = Record<
      string,
      {
        tableType: TableType;
        keyLabel: string;
        valueLabel: string;
        rows: Array<{ key: string; value: string }>;
      }
    >;
    const magicTables = onlyOks(data.magicTables).reduce(
      (acc: TableRow, row) => {
        const tableData = acc[row.table_id];
        if (tableData) {
          tableData.rows.push({
            key: row.key,
            value: row.value,
          });
        } else {
          if (isNil(row.table_type)) {
            throw new Error(
              `Failed to create a table called '${row.table_id}', first row definition for this table needs to have a table_type`,
            );
          }
          acc[row.table_id] = {
            tableType: row.table_type,
            keyLabel: row.key,
            valueLabel: row.value,
            rows: [],
          };
        }
        return acc;
      },
      {},
    );
    const magicTableMap: IdMap = new Map(
      await Promise.all(
        toPairs(magicTables).map(async ([label, table]) => {
          const tableId = await createInfoTable(
            label,
            table.tableType,
            table.keyLabel,
            table.valueLabel,
          );
          await createInfoTableRows(tableId, table.rows);
          trackProgress(table.rows.length + 1);
          return [label, tableId] as const;
        }),
      ),
    );

    currentWork = 'Traditions';

    const traditionMap: IdMap = new Map(
      await Promise.all(
        onlyOks(data.magicTraditions).map(async (record) => {
          const magicTableId = record.table
            ? magicTableMap.get(record.table)
            : null;
          if (magicTableId === undefined) {
            // but not null
            throw new Error(
              `Failed to associate a tradition with a table named '${record.table}', no such table was found`,
            );
          }
          const traditionId = await createTradition(record, magicTableId);
          trackProgress();
          return [record.name, traditionId] as const;
        }),
      ),
    );

    currentWork = 'Magic Talents';

    const magicReferenceGetter = getMagicReferences(
      traditionMap,
      magicTableMap,
      optionBlockMap,
    );
    const magicTalents = onlyOks(data.magicTalents);
    const talentActivations = uniq(
      flatten(magicTalents.map((t) => t.activate)),
    );
    const talentActivationMap: IdMap = new Map(
      await Promise.all(
        talentActivations.map(async (activation) => {
          const activationId = await createActivationTag(activation);
          return [activation, activationId] as const;
        }),
      ),
    );

    await Promise.all(
      magicTalents.map(async (record) => {
        const [traditionId, magicTableId, optionBlockId] =
          magicReferenceGetter(
            record.tradition,
            record.table,
            record.options,
          );

        const magicTalentId = await createMagicTalent(
          record,
          traditionId,
          magicTableId,
          optionBlockId,
        );

        await Promise.all(
          record.activate.map(
            associateTalentActivations(
              talentActivationMap,
              magicTalentId,
              record.talent_name,
            ),
          ),
        );

        trackProgress();
        return [record.talent_name, magicTalentId] as const;
      }),
    );

    currentWork = 'Magic Spells';

    await Promise.all(
      onlyOks(data.magicSpells).map(async (record) => {
        const [traditionId, magicTableId, optionBlockId] = magicReferenceGetter(
          record.tradition,
          record.table,
          record.options,
        );

        await createMagicSpell(
          record,
          traditionId,
          magicTableId,
          optionBlockId,
        );

        trackProgress();
      }),
    );

    return ok(totalRecords);
  } catch (error) {
    return err({
      title: 'DB Import Error',
      message: `${error}`,
    });
  }
};

const onlyOks = <T>(res: Array<Result<T, unknown>>): T[] => {
  return res.reduce((acc: T[], r) => {
    r.map((value) => acc.push(value));
    return acc;
  }, []);
};

const countOk = (records: Array<Result<unknown, string[]>>): number => {
  return records.filter((r) => r.isOk()).length;
};

// Not totally accurate because some minor records are determined on the fly
// (like immunities), but this will determine roughly how many total static
// records will be created, which is good enough.
export const countRecords = (data: ImportData) => {
  return sum(values(data).map(countOk));
};

const UNIT_AMOUNT_REGEX = new RegExp(/(?<label>[\w ]+)\((?<amount>.+)\)/);
const extractUnitAmount = (labelAndAmount: string): [string, string | null] => {
  const { label, amount } =
    labelAndAmount.match(UNIT_AMOUNT_REGEX)?.groups || {};
  return amount ? [label.trim(), amount.trim()] : [labelAndAmount.trim(), null];
};

const destroyAll = async () => {
  // This could technically be a modicum faster if these were batched up such that
  // required foreign references were respected, but it would only save on overhead,
  // so this will be fast enough.
  for (const table of CLEAR_TABLES) {
    await dbExecute(`DELETE FROM ${table}`);
  }
};

// This looks a little horrendous, but hey, that's DRY
const checkAssociationId =
  (centralTable: string) =>
  (
    tableLabel: string,
    recordLabel: string,
    centralRecordLabel: string,
    id?: number,
  ): id is number => {
    if (isNil(id)) {
      throw new Error(
        `Failed to associate a ${tableLabel} named '${recordLabel}' with a ${centralTable} named '${centralRecordLabel}', no '${recordLabel}' ${tableLabel} record was found`,
      );
    }
    return true;
  };

const checkAncestryAssociationId = checkAssociationId('ancestry');
const checkLevelAssociationId = checkAssociationId('level');
const checkTalentAssociationId = checkAssociationId('talent');

const associateAncestryLanguage =
  (languageMap: IdMap, ancestryId: number, ancestryName: string) =>
  async (language: string) => {
    const langId = languageMap.get(language);
    if (
      checkAncestryAssociationId('language', language, ancestryName, langId)
    ) {
      await createAncestryLanguage(ancestryId, langId);
    }
  };

const associateAncestrySpeedTraits =
  (speedTraitMap: IdMap, ancestryId: number, ancestryName: string) =>
  async (trait: string) => {
    const [traitLabel, traitAmount] = extractUnitAmount(trait);
    const speedTraitId = speedTraitMap.get(traitLabel);
    if (
      checkAncestryAssociationId(
        'speed trait',
        traitLabel,
        ancestryName,
        speedTraitId,
      )
    ) {
      await createAncestrySpeedTrait(ancestryId, speedTraitId, traitAmount);
    }
  };

const associateAncestrySenses =
  (senseMap: IdMap, ancestryId: number, ancestryName: string) =>
  async (sense: string) => {
    const [senseLabel, senseAmount] = extractUnitAmount(sense);
    const senseId = senseMap.get(senseLabel);
    if (
      checkAncestryAssociationId('sense', senseLabel, ancestryName, senseId)
    ) {
      await createAncestrySense(ancestryId, senseId, senseAmount);
    }
  };

const associateAncestryImmunity =
  (immunityMap: IdMap, ancestryId: number, ancestryName: string) =>
  async (immunity: string) => {
    const immunityId = immunityMap.get(immunity);
    if (
      checkAncestryAssociationId('immunity', immunity, ancestryName, immunityId)
    ) {
      await createAncestryImmunity(ancestryId, immunityId);
    }
  };

const associateLevelLanguages =
  (languageMap: IdMap, levelId: number, levelLabel: string) =>
  async (language: string) => {
    const langId = languageMap.get(language);
    if (checkLevelAssociationId('language', language, levelLabel, langId)) {
      await createLevelLanguage(levelId, langId);
    }
  };

const associateLevelSpeedTraits =
  (speedTraitMap: IdMap, levelId: number, levelLabel: string) =>
  async (trait: string) => {
    const [traitLabel, traitAmount] = extractUnitAmount(trait);
    const traitId = speedTraitMap.get(traitLabel);
    if (
      checkLevelAssociationId('speed trait', traitLabel, levelLabel, traitId)
    ) {
      await createLevelSpeedTrait(levelId, traitId, traitAmount);
    }
  };

const associateTalentActivations =
  (talentActivationMap: IdMap, talentId: number, talentLabel: string) =>
  async (activation: string) => {
    const activationId = talentActivationMap.get(activation);
    if (
      checkTalentAssociationId(
        'activate tag',
        activation,
        talentLabel,
        activationId,
      )
    ) {
      await createMagicTalentActivation(talentId, activationId);
    }
  };

// This set of references is needed twice the same way, so I'm DRYing it
const getMagicReferences =
  (traditionMap: IdMap, magicTableMap: IdMap, optionBlockMap: IdMap) =>
  (tradition: string, magicTable: string | null, options: string | null) => {
    const traditionId = traditionMap.get(tradition);
    if (!traditionId) {
      throw new Error(
        `Failed to associate a magic talent with a tradition named '${tradition}', no such tradition was found`,
      );
    }

    const magicTableId = magicTable ? magicTableMap.get(magicTable) : null;
    if (magicTableId === undefined) {
      // but not null
      throw new Error(
        `Failed to associate a magic talent with a table named '${magicTable}', no such table was found`,
      );
    }

    const optionBlockId = options ? optionBlockMap.get(options) : null;
    if (optionBlockId === undefined) {
      // but not null
      throw new Error(
        `Failed to associate a magic talent with options named '${options}', no such option block was found`,
      );
    }

    return [traditionId, magicTableId, optionBlockId] as const;
  };
