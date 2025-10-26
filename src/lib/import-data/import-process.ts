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
} from '../db/junctions';
import { CLEAR_TABLES, dbExecute } from '../db/client';

type IdMap = Map<string, number>;

export const importProcess = async (
  data: ImportData,
  onProgress: (workingOn: string, percent: number) => void,
) => {
  await destroyAll();

  const totalRecords = countRecords(data);
  let currentRecords = 0;
  let currentPercentage = -1;
  let currentTable = 'Profession Categories';

  const trackProgress = () => {
    currentRecords++;
    const newPercent = Math.floor((currentRecords / totalRecords) * 100);
    if (newPercent > currentPercentage) {
      currentPercentage = newPercent;
      onProgress(currentTable, currentPercentage);
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

  currentTable = 'Professions';

  await Promise.all(
    onlyOks(data.professions).map(async (record) => {
      await createProfession(record);
      trackProgress();
    }),
  );

  currentTable = 'Languages';

  const languageMap: IdMap = new Map(
    await Promise.all(
      onlyOks(data.languages).map(async (record) => {
        const langId = await createLanguage(record);
        trackProgress();
        return [record.name, langId] as const;
      }),
    ),
  );

  currentTable = 'Speed Traits';

  const speedTraitMap: IdMap = new Map(
    await Promise.all(
      onlyOks(data.speedTraits).map(async (record) => {
        const speedTraitId = await createSpeedTrait(record);
        trackProgress();
        return [record.name, speedTraitId] as const;
      }),
    ),
  );

  currentTable = 'Senses';

  const senseMap: IdMap = new Map(
    await Promise.all(
      onlyOks(data.senses).map(async (record) => {
        const senseId = await createSense(record);
        trackProgress();
        return [record.name, senseId] as const;
      }),
    ),
  );

  currentTable = 'Ancestries';

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

  await Promise.all(
    ancestries.map(async (ancestry) => {
      const ancestryId = await createAncestry(ancestry);
      const ancestryName = ancestry.ancestry;

      await Promise.all([
        ...ancestry.languages.map(
          associateLanguage(languageMap, ancestryId, ancestryName),
        ),
        ...ancestry.speed_traits.map(
          associateSpeedTraits(speedTraitMap, ancestryId, ancestryName),
        ),
        ...ancestry.senses.map(
          associatedSenses(senseMap, ancestryId, ancestryName),
        ),
        ...ancestry.immunities.map(
          associateImmunity(immunityMap, ancestryId, ancestryName),
        ),
      ]);
    }),
  );

  console.log(speedTraitMap);
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

const checkAncestryAssociationId = (tableLabel: string, recordLabel: string, ancestryName: string, id?: number): id is number => {
  if (isNil(id)) {
    throw new Error(
      `Tried to associate a ${tableLabel} named '${recordLabel}' with ancestry '${ancestryName}', but no '${recordLabel}' ${tableLabel} record was found`,
    );
  }
  return true;
};

const associateLanguage =
  (languageMap: IdMap, ancestryId: number, ancestryName: string) =>
  async (lang: string) => {
    const langId = languageMap.get(lang);
    if (checkAncestryAssociationId('language', lang, ancestryName, langId)) {
      await createAncestryLanguage(ancestryId, langId);
    }
  };

const associateSpeedTraits =
  (speedTraitMap: IdMap, ancestryId: number, ancestryName: string) =>
  async (trait: string) => {
    const [traitLabel, traitAmount] = extractUnitAmount(trait);
    const speedTraitId = speedTraitMap.get(traitLabel);
    if (checkAncestryAssociationId('speed trait', traitLabel, ancestryName, speedTraitId)) {
      await createAncestrySpeedTrait(ancestryId, speedTraitId, traitAmount);
    }
  };

const associatedSenses =
  (senseMap: IdMap, ancestryId: number, ancestryName: string) =>
  async (sense: string) => {
    const [senseLabel, senseAmount] = extractUnitAmount(sense);
    const senseId = senseMap.get(senseLabel);
    if (checkAncestryAssociationId('sense', senseLabel, ancestryName, senseId)) {
      await createAncestrySense(ancestryId, senseId, senseAmount);
    }
  };

const associateImmunity =
  (immunityMap: IdMap, ancestryId: number, ancestryName: string) =>
  async (immunity: string) => {
    const immunityId = immunityMap.get(immunity);
    if (checkAncestryAssociationId('immunity', immunity, ancestryName, immunityId)) {
      await createAncestryImmunity(ancestryId, immunityId);
    }
  };
