import zod from 'zod';
import { parseRow, Validations } from './import-utils';
import { CsvRawParseResult } from '.';

const ProfessionCategoryValidator = zod.object({
  name: Validations.STRING,
  description: Validations.STRING,
});

const ProfessionValidator = zod.object({
  name: Validations.STRING,
  category: Validations.STRING,
  description: Validations.STRING,
});

export type ProfessionCategoryRecord = zod.output<
  typeof ProfessionCategoryValidator
>;
export type ProfessionRecord = zod.output<typeof ProfessionValidator>;

export const parseProfessionCategoryCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(ProfessionCategoryValidator));

export const parseProfessionCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(ProfessionValidator));
