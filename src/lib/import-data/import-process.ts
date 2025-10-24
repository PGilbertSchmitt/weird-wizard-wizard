// import { CSVParseResults } from './import-utils';
import { ImportData } from '.';
import { Result } from 'neverthrow';
import { flatten, sum, uniq, values } from 'ramda';
import { createProfession, createProfessionCategory } from '../db/professions';
import {
  createImmunity,
  createLanguage,
  createSense,
  createSpeedTrait,
} from '../db/natural-attributes';
import { createAncestry } from '../db/ancestries';
import { createAncestryImmunity, createAncestryLanguage, createAncestrySpeedTrait } from '../db/junctions';
import { dbExecute, Tables } from '../db/client';

type IdMap = Map<string, number>;

const onlyOks = <T>(res: Array<Result<T, unknown>>): T[] => {
  return res.reduce((acc: T[], r) => {
    r.map((value) => acc.push(value));
    return acc;
  }, []);
};

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
  const immunities = uniq(flatten(ancestries.map(a => a.immunities)));
  const immunityMap: IdMap = new Map(await Promise.all(immunities.map(async immunity => {
    const immunityId = await createImmunity(immunity);
    return [immunity, immunityId] as const;
  })));

  const associateLanguage = (ancestryId: number, ancestryName: string) => async (lang: string) => {
    const id = languageMap.get(lang);
    if (!id) {
      throw new Error(`Tried to associate language '${lang}' with ancestry '${ancestryName}', but no such language was found`);
    }
    await createAncestryLanguage(ancestryId, id);
  };

  // const associateSpeedTraits = (ancestryId: number, ancestryName: string) => async (trait: string) => {
  //   const id = speedTraitMap.get(trait);
  //   if (!id) {
  //     throw new Error(`Tried to associate speed trait '${trait}' with ancestry '${ancestryName}', but no such speed trait was found`)
  //   }
  //   await createAncestrySpeedTrait(ancestryId, id);
  // };

  const associateImmunity = (ancestryId: number, ancestryName: string) => async (immunity: string) => {
    const id = immunityMap.get(immunity);
    if (!id) {
      throw new Error(`Tried to associate immunity '${immunities}' with ancestry '${ancestryName}', but no such immunity was found`);
    }
    await createAncestryImmunity(ancestryId, id);
  };

  await Promise.all(
    ancestries.map(async (ancestry) => {
      const ancestryId = await createAncestry(ancestry);

      await Promise.all([
        ...ancestry.languages.map(associateLanguage(ancestryId, ancestry.ancestry)),
        // ...ancestry.speed_traits.map(associateSpeedTraits(ancestryId, ancestry.ancestry)),
        // ...ancestry.senses.map(),
        ...ancestry.immunities.map(associateImmunity(ancestryId, ancestry.ancestry)),
      ]);
    }),
  );

  console.log(languageMap, speedTraitMap, senseMap);
};

const countOk = (records: Array<Result<unknown, string[]>>): number => {
  return records.filter((r) => r.isOk()).length;
};

// Not totally accurate because some minor records are determined on the fly
// (like immunities), but this will determine roughly how many total static
// recordswill be created.
export const countRecords = (data: ImportData) => {
  return sum(values(data).map(countOk));
};

const destroyAll = async () => {
  console.log(values(Tables).map(table => (`DELETE FROM ${table}`)))
  await Promise.all(values(Tables).map(table => dbExecute(`DELETE FROM ${table}`)));
};
