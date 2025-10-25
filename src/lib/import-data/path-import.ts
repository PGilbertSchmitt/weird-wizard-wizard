import zod from 'zod';
import { parseRow, Validations } from './import-utils';
import { CsvRawParseResult } from '.';

const NovicePathValidator = zod.object({
  name: Validations.STRING,
  description: Validations.STRING,
  init_scores_lvl_1: Validations.STRING.regex(
    /^[\d]+\|[\d]+\|[\d]+\|[\d]+$/,
  ).transform((scores) => {
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

const NonNovicePathValidator = zod.object({
  name: Validations.STRING,
  description: Validations.STRING,
  sub_path: Validations.STRING,
});

export type NovicePathRecord = zod.output<typeof NovicePathValidator>;
export type NonNovicePathRecord = zod.output<typeof NonNovicePathValidator>;

export const parseNovicePathCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(NovicePathValidator));

export const parseNonNovicePathCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(NonNovicePathValidator));
