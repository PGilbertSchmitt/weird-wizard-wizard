import Database, { QueryResult } from '@tauri-apps/plugin-sql';
import { isNil } from 'ramda';
import { ValueOf } from '../types';

// Could be more robust
const dbPromise = Database.load('sqlite:www.db');

export const dbSelect = async (query: string, bindValues?: unknown[]) => {
  return (await dbPromise).select(query, bindValues);
};

export const dbExecute = async (query: string, bindValues?: unknown[]) => {
  return (await dbPromise).execute(query, bindValues);
};

export const id = (table: TableName, { lastInsertId }: QueryResult) => {
  if (isNil(lastInsertId)) {
    throw new Error(`Failed to create record on table: ${table}`);
  }

  return lastInsertId;
};

// There are enough name+description tables to make this useful
export const nameAndDescQuery = async (
  table: TableName,
  record: { name: string; description: string },
) =>
  id(
    table,
    await dbExecute(
      `
INSERT INTO ${table} (
  name, description
) VALUES ($1, $2)
  ON CONFLICT(name) DO UPDATE SET description=excluded.description`,
      [record.name, record.description],
    ),
  );

export const Tables = {
  LANGUAGES: 'languages',
  SPEED_TRAITS: 'speed_traits',
  SENSES: 'senses',
  IMMUNITIES: 'immunities',
  PROFESSION_CATEGORIES: 'profession_categories',
  PROFESSIONS: 'professions',
  ANCESTRIES: 'ancestries',
  ANCESTRY_LANGUAGES: 'ancestry_languages',
  ANCESTRY_SPEED_TRAITS: 'ancestry_speed_traits',
  ANCESTRY_SENSES: 'ancestry_senses',
  ANCESTRY_IMMUNITIES: 'ancestry_immunities',
  PATHS: 'paths',
  LEVELS: 'levels',
  LEVEL_TALENTS: 'level_talents',
  LEVEL_TRADITIONS: 'level_traditions',
  LEVEL_LANGUAGES: 'level_languages',
  TALENTS: 'talents',
  SPELLS: 'spells',
  TRADITIONS: 'traditions',
  TRADITION_SPECIAL_INFO: 'tradition_special_info',
  INFO_TABLES: 'info_tables',
  INFO_TABLE_ROW: 'info_table_row',
  OPTION_BLOCKS: 'option_blocks',
  OPTION_BLOCK_ROWS: 'option_block_rows',
  ACTIVATE_TAGS: 'activate_tags',
  TALENT_ACTIVATIONS: 'talent_activations',
  CHARACTERS: 'characters',
} as const;

export type TableName = ValueOf<typeof Tables>;

// Ordered to be deleted cleanly since many foreign keys are required
export const CLEAR_TABLES: TableName[] = [
  // Currently, imports will wipe characters, but it could be possible to
  // reconstruct them after re-importing them using other labels.
  'characters',

  // Junctions between ancestries and attribute tables
  'ancestry_immunities',
  'ancestry_languages',
  'ancestry_senses',
  'ancestry_speed_traits',

  // Junctions between levels and attribute tables
  'level_languages',
  'level_talents',
  'level_traditions',

  // References paths
  'levels',

  // References ancestries
  'paths',

  // References traditions
  'tradition_special_info',

  // References talents
  'talent_activations',

  // References info_tables, option_blocks, or both
  'talents',
  'spells',
  'traditions',
  'info_table_row',
  'option_block_rows',
  
  // No dependencies
  'profession_categories',
  'professions',
  'ancestries',
  'info_tables',
  'option_blocks',
  'activate_tags',
  'immunities',
  'languages',
  'senses',
  'speed_traits',
];
