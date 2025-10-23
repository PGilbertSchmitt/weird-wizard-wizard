import zod from 'zod';
import { CSVParseResults, parseRow, Validations } from './import-utils';
import { CsvRawParseResult } from '.';

const MagicTraditionValidator = zod.object({
  name: Validations.STRING,
  table: zod.string(),
  blurb: Validations.STRING,
  special_info: zod.string(),
  description: Validations.STRING,
});

const MagicTalentValidator = zod.object({
  tradition: Validations.STRING,
  talent_name: Validations.STRING,
  charges: zod.literal(['', '1', '1:1|2:3|3:7']),
  restore: Validations.STRING,
  activate: Validations.STRING,
  table: zod.string(),
  options: zod.string(),
  description: Validations.STRING,
});

const MagicSpellValidator = zod.object({
  name: Validations.STRING,
  tradition: Validations.STRING,
  path_type: zod.literal(['Novice', 'Expert', 'Master']),
  castings: Validations.POS_NUM,
  duration: Validations.STRING,
  target: Validations.STRING,
  condition: zod.string(),
  ritual: zod.literal(['', 'TRUE', 'FALSE']),
  table: zod.string(),
  options: zod.string(),
  description: Validations.STRING,
});

const MagicTableValidator = zod.object({
  table_id: Validations.STRING,
  key: Validations.STRING,
  value: Validations.STRING,
  table_type: zod.literal(['', 'BLOCK', 'TABLE', 'ROLL']),
});

const MagicOptionValidator = zod.object({
  options_id: Validations.STRING,
  description: Validations.STRING,
});

type MagicTraditionRecord = zod.output<typeof MagicTraditionValidator>;
type MagicTalentRecord = zod.output<typeof MagicTalentValidator>;
type MagicSpellRecord = zod.output<typeof MagicSpellValidator>;
type MagicTableRecord = zod.output<typeof MagicTableValidator>;
type MagicOptionRecord = zod.output<typeof MagicOptionValidator>;

export const parseMagicTraditionCSV = (
  data: CsvRawParseResult,
): CSVParseResults<MagicTraditionRecord> => {
  return data.map(parseRow(MagicTraditionValidator));
};

export const parseMagicTalentCSV = (
  data: CsvRawParseResult,
): CSVParseResults<MagicTalentRecord> => {
  return data.map(parseRow(MagicTalentValidator));
};

export const parseMagicSpellCSV = (
  data: CsvRawParseResult,
): CSVParseResults<MagicSpellRecord> => {
  return data.map(parseRow(MagicSpellValidator));
};

export const parseMagicTableCSV = (
  data: CsvRawParseResult,
): CSVParseResults<MagicTableRecord> => {
  return data.map(parseRow(MagicTableValidator));
};

export const parseMagicOptionCSV = (
  data: CsvRawParseResult,
): CSVParseResults<MagicOptionRecord> => {
  return data.map(parseRow(MagicOptionValidator));
};
