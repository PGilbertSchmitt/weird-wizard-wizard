// import { CSVParseResults } from './import-utils';
import { ImportData } from '.';
import { Result } from 'neverthrow';
import { sum, values } from 'ramda';
import { createProfession, createProfessionCategory } from '../db/professions';
import {
  createLanguage,
  createSense,
  createSpeedTrait,
} from '../db/natural-attributes';

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
