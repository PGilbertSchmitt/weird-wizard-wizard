import Database from '@tauri-apps/plugin-sql';

// Could be more robust
const dbPromise = Database.load('sqlite:www.db');

export const dbSelect = async (query: string, bindValues?: unknown[]) => {
  return (await dbPromise).select(query, bindValues);
};

export const dbExecute = async (query: string, bindValues?: unknown[]) => {
  return (await dbPromise).execute(query, bindValues);
};
