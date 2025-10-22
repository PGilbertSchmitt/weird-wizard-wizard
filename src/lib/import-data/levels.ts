import zod from 'zod';
import { CSVParseResults, parseRow, parseCSV, Validations } from './import-utils';

const LevelValidator = zod.object({
  path: zod.string(),
  size: Validations.SIZE.optional(),
  level: Validations.POS_NUM,
  health: Validations.POS_NUM,
  nat_def: Validations.POS_OR_ZERO,
  armed_def: Validations.POS_OR_ZERO,
  speed: Validations.POS_OR_ZERO,
  bonus_dmg: Validations.POS_OR_ZERO,
  trad_choices: Validations.POS_OR_ZERO,
  novice_spells: Validations.POS_OR_ZERO,
  expert_spells: Validations.POS_OR_ZERO,
  master_spells: Validations.POS_OR_ZERO,
  lang_choices: Validations.POS_OR_ZERO,
  languages: Validations.PIPE_DELIM_ARRAY,
  speed_traits: Validations.PIPE_DELIM_ARRAY,
  talents: Validations.PIPE_DELIM_ARRAY,
});

export type LevelRecord = zod.output<typeof LevelValidator>;

export const parseLevelCSV = (data: Uint8Array): CSVParseResults<LevelRecord> => {
  return parseCSV(data).map(parseRow(LevelValidator));
};
