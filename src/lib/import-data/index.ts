import { parseAncestryCSV } from "./ancestry";
import { parseLevelCSV } from "./levels";

export const CsvFiles = {
  'ancestries.csv': 'Ancestries',
  'expert_levels.csv': 'ExpertLevels',
  'expert_paths.csv': 'ExpertPaths',
  'magic_options.csv': 'MagicOptions',
  'magic_spells.csv': 'MagicSpells',
  'magic_tables.csv': 'MagicTables',
  'magic_talents.csv': 'MagicTalents',
  'magic_traditions.csv': 'MagicTraditions',
  'master_levels.csv': 'MasterLevels',
  'master_paths.csv': 'MasterPaths',
  'novice_levels.csv': 'NoviceLevels',
  'novice_paths.csv': 'NovicePaths',
} as const;

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
};

export type CsvBuffers = Record<keyof typeof BufferFiles, Uint8Array>;

export interface ImportData {
  ancestries: ReturnType<typeof parseAncestryCSV>;
  noviceLevels: ReturnType<typeof parseLevelCSV>;
  expertLevels: ReturnType<typeof parseLevelCSV>;
  masterLevels: ReturnType<typeof parseLevelCSV>;
  novicePaths: unknown;
  expertPaths: unknown;
  masterPaths: unknown;
  magicTraditions: unknown;
  magicTalents: unknown;
  magicSpells: unknown;
  magicTables: unknown;
  magicOptions: unknown;
}
