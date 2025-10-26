// import { CSVParseResults } from './import-utils';
import { ImportData } from '.';
import { Result } from 'neverthrow';
import { flatten, isNil, sum, uniq, values } from 'ramda';
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
} from '../db/junctions';
import { CLEAR_TABLES, dbExecute } from '../db/client';
import { createNonNovicePath, createNovicePath } from '../db/paths';
import { PathKinds } from '../db/enums';
import { createLevel } from '../db/levels';

type IdMap = Map<string, number>;

export const importProcess = async (
  data: ImportData,
  onProgress: (workingOn: string, percent: number) => void,
) => {
  await destroyAll();

  const totalRecords = countRecords(data);
  let currentRecords = 0;
  let currentPercentage = -1;
  let currentWork = 'Profession';

  const trackProgress = () => {
    currentRecords++;
    const newPercent = Math.floor((currentRecords / totalRecords) * 100);
    if (newPercent > currentPercentage) {
      currentPercentage = newPercent;
      onProgress(currentWork, currentPercentage);
    }
  };

  // Initial progress update
  trackProgress();

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
        return [novicePath.name, pathId] as const;
      }),
      ...onlyOks(data.expertPaths).map(async (expertPath) => {
        const pathId = await createNonNovicePath(expertPath, PathKinds.EXPERT);
        return [expertPath.name, pathId] as const;
      }),
      ...onlyOks(data.masterPaths).map(async (masterPath) => {
        const pathId = await createNonNovicePath(masterPath, PathKinds.MASTER);
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
          `Failed to associate a level with path named '${level.path}', no such such path was found`,
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
    }),
  );

  console.log('Done!');
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
