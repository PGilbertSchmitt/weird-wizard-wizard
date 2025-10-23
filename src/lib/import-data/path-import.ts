import zod from 'zod';
import { CSVParseResults, parseRow, Validations } from './import-utils';
import { CsvRawParseResult } from '.';

const NovicePathValidator = zod.object({
  name: Validations.STRING,
  description: Validations.STRING,
  init_scores_lvl_1: zod
    .string()
    .regex(/^[\d]+\|[\d]+\|[\d]+\|[\d]+$/)
    .transform((scores) => {
      const [str, agl, int, will] = scores.split('|');
      return {
        strength: parseInt(str),
        agility: parseInt(agl),
        intellect: parseInt(int),
        will: parseInt(will),
      };
    }),
  origin_locked: Validations.BOOL,
});

export type NovicePathRecord = zod.output<typeof NovicePathValidator>;

export const parseNovicePathCSV = (
  data: CsvRawParseResult,
): CSVParseResults<NovicePathRecord> => {
  return data.map(parseRow(NovicePathValidator));
};

const NonNovicePathValidator = zod.object({
  name: Validations.STRING,
  description: Validations.STRING,
  sub_path: Validations.STRING,
});

export type NonNovicePathRecord = zod.output<typeof NonNovicePathValidator>;

export const parseNonNovicePathCSV = (
  data: CsvRawParseResult,
): CSVParseResults<NonNovicePathRecord> => {
  return data.map(parseRow(NonNovicePathValidator));
};
