import zod from 'zod';
import { parseRow, Validations } from './import-utils';
import { CsvRawParseResult } from '.';

const AncestryValidator = zod.object({
  ancestry: Validations.STRING,
  descriptor: Validations.STRING_OPT,
  size: Validations.SIZE,
  speed: Validations.POS_NUM,
  add_health: Validations.POS_OR_ZERO,
  nat_def: Validations.POS_OR_ZERO,
  languages: Validations.PIPE_DELIM_ARRAY,
  speed_traits: Validations.PIPE_DELIM_ARRAY,
  senses: Validations.PIPE_DELIM_ARRAY,
  immunities: Validations.PIPE_DELIM_ARRAY,
  traits: Validations.PIPE_DELIM_ARRAY,
});

export type AncestryRecord = zod.output<typeof AncestryValidator>;

export const parseAncestryCSV = (data: CsvRawParseResult) =>
  data.map(parseRow(AncestryValidator));
