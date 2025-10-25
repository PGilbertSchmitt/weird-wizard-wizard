import zod from 'zod';
import { parseRow, Validations } from './import-utils';
import { CsvRawParseResult } from '.';
import { values } from 'ramda';
import { Units } from '../db/enums';

const LanguageValidator = zod.object({
  name: Validations.STRING,
  description: Validations.STRING,
  secret: Validations.BOOL,
});

const SpeedTraitValidator = zod.object({
  name: Validations.STRING,
  description: Validations.STRING,
  unit: zod
    .enum(values(Units))
    .nullable(),
});

const SenseValidator = zod.object({
  name: Validations.STRING,
  description: Validations.STRING,
});

export type LanguageRecord = zod.output<typeof LanguageValidator>;
export type SpeedTraitRecord = zod.output<typeof SpeedTraitValidator>;
export type SenseRecord = zod.output<typeof SenseValidator>;

export const parseLanguageCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(LanguageValidator));

export const parseSpeedTraitCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(SpeedTraitValidator));

export const parseSenseCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(SenseValidator));
