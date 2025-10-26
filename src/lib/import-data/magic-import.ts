import zod from 'zod';
import { parseRow, Validations } from './import-utils';
import { CsvRawParseResult } from '.';
import {
  MagicTalentCharge,
  MagicTalentCharges,
  PathKinds,
  TableTypes,
} from '../db/enums';
import { values } from 'ramda';

const MagicTraditionValidator = zod.object({
  name: Validations.STRING,
  table: Validations.STRING_OPT,
  blurb: Validations.STRING,
  special_info: Validations.STRING_OPT,
  description: Validations.STRING,
});

const MagicTalentValidator = zod.object({
  tradition: Validations.STRING,
  talent_name: Validations.STRING,
  charges: zod
    .enum(['1', '1:1|2:3|3:7'])
    .nullable()
    .transform((value): MagicTalentCharge => {
      switch (value) {
        case '1':
          return MagicTalentCharges.ONE;
        case '1:1|2:3|3:7':
          return MagicTalentCharges.ONE_TWO_THREE;
        default:
          return MagicTalentCharges.NONE;
      }
    }),
  restore: Validations.STRING,
  activate: Validations.STRING,
  table: Validations.STRING_OPT,
  options: Validations.STRING_OPT,
  description: Validations.STRING,
});

const MagicSpellValidator = zod.object({
  name: Validations.STRING,
  tradition: Validations.STRING,
  path_type: zod.enum(values(PathKinds)),
  castings: Validations.POS_NUM,
  duration: Validations.STRING,
  target: Validations.STRING,
  condition: Validations.STRING_OPT,
  ritual: Validations.BOOL,
  table: Validations.STRING_OPT,
  options: Validations.STRING_OPT,
  description: Validations.STRING,
});

const MagicTableValidator = zod.object({
  table_id: Validations.STRING,
  key: Validations.STRING,
  value: Validations.STRING,
  table_type: zod.enum(values(TableTypes)).nullable(),
});

const MagicOptionValidator = zod.object({
  options_id: Validations.STRING,
  description: Validations.STRING,
});

export type MagicTraditionRecord = zod.output<typeof MagicTraditionValidator>;
export type MagicTalentRecord = zod.output<typeof MagicTalentValidator>;
export type MagicSpellRecord = zod.output<typeof MagicSpellValidator>;
export type MagicTableRecord = zod.output<typeof MagicTableValidator>;
export type MagicOptionRecord = zod.output<typeof MagicOptionValidator>;

export const parseMagicTraditionCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(MagicTraditionValidator));

export const parseMagicTalentCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(MagicTalentValidator));

export const parseMagicSpellCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(MagicSpellValidator));

export const parseMagicTableCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(MagicTableValidator));

export const parseMagicOptionCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(MagicOptionValidator));
