import Database, { QueryResult } from '@tauri-apps/plugin-sql';
import { isNil } from 'ramda';

// Could be more robust
const dbPromise = Database.load('sqlite:www.db');

export const dbSelect = async (query: string, bindValues?: unknown[]) => {
  return (await dbPromise).select(query, bindValues);
};

export const dbExecute = async (query: string, bindValues?: unknown[]) => {
  return (await dbPromise).execute(query, bindValues);
};

export const id = (label: string, { lastInsertId }: QueryResult) => {
  if (isNil(lastInsertId)) {
    throw new Error(`Failed to create record on table: ${label}`);
  }

  return lastInsertId;
};

// There are enough name+description tables to make this useful
export const nameAndDescQuery = async (
  table: string,
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
