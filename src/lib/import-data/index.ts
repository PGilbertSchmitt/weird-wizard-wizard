import { values } from 'ramda';
import { parseAncestryCSV } from './ancestry-import';
import { parseLevelCSV } from './level-import';
import {
  parseMagicOptionCSV,
  parseMagicSpellCSV,
  parseMagicTableCSV,
  parseMagicTalentCSV,
  parseMagicTraditionCSV,
} from './magic-import';
import {
  parseLanguageCSV,
  parseSenseCSV,
  parseSpeedTraitCSV,
} from './natural-attribute-import';
import { parseNonNovicePathCSV, parseNovicePathCSV } from './path-import';
import {
  parseProfessionCategoryCSV,
  parseProfessionCSV,
} from './profession-import';

export const BufferFiles = {
  ancestriesBuffer: 'ancestries.csv',
  expertLevelsBuffer: 'expert_levels.csv',
  expertPathsBuffer: 'expert_paths.csv',
  magicOptionsBuffer: 'magic_options.csv',
  magicSpellsBuffer: 'magic_spells.csv',
  magicTablesBuffer: 'magic_tables.csv',
  magicTalentsBuffer: 'magic_talents.csv',
  magicTraditionsBuffer: 'magic_traditions.csv',
  masterLevelsBuffer: 'master_levels.csv',
  masterPathsBuffer: 'master_paths.csv',
  noviceLevelsBuffer: 'novice_levels.csv',
  novicePathsBuffer: 'novice_paths.csv',
  languagesBuffer: 'languages.csv',
  speedTraitsBuffer: 'speed_traits.csv',
  sensesBuffer: 'senses.csv',
  professionCategoriesBuffer: 'profession_categories.csv',
  professionsBuffer: 'professions.csv',
} as const;

export const CsvFiles = values(BufferFiles);

export type CsvRawParseResult = Array<Record<string, string>>;
export type CsvResults = Record<keyof typeof BufferFiles, CsvRawParseResult>;

export interface ImportData {
  ancestries: ReturnType<typeof parseAncestryCSV>;
  noviceLevels: ReturnType<typeof parseLevelCSV>;
  expertLevels: ReturnType<typeof parseLevelCSV>;
  masterLevels: ReturnType<typeof parseLevelCSV>;
  novicePaths: ReturnType<typeof parseNovicePathCSV>;
  expertPaths: ReturnType<typeof parseNonNovicePathCSV>;
  masterPaths: ReturnType<typeof parseNonNovicePathCSV>;
  magicTraditions: ReturnType<typeof parseMagicTraditionCSV>;
  magicTalents: ReturnType<typeof parseMagicTalentCSV>;
  magicSpells: ReturnType<typeof parseMagicSpellCSV>;
  magicTables: ReturnType<typeof parseMagicTableCSV>;
  magicOptions: ReturnType<typeof parseMagicOptionCSV>;
  languages: ReturnType<typeof parseLanguageCSV>;
  speedTraits: ReturnType<typeof parseSpeedTraitCSV>;
  senses: ReturnType<typeof parseSenseCSV>;
  professionCategories: ReturnType<typeof parseProfessionCategoryCSV>;
  professions: ReturnType<typeof parseProfessionCSV>;
}
