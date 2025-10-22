import zod from 'zod';
import {
  CSVParseResults,
  parseRow,
  parseCSV,
  Validations,
} from './import-utils';

const AncestryValidator = zod.object({
  ancestry: zod.string(),
  descriptor: zod.string(),
  size: Validations.SIZE_REQUIRED,
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

export const parseAncestryCSV = (
  data: Uint8Array,
): CSVParseResults<AncestryRecord> => {
  return parseCSV(data).map(parseRow(AncestryValidator));
};
