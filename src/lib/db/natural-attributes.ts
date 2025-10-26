import {
  LanguageRecord,
} from '../import-data/natural-attribute-import';
import { dbExecute, id, TableName, Tables } from './client';

export const createLanguage = async (record: LanguageRecord) => {
  return id(
    Tables.LANGUAGES,
    await dbExecute(
      `
INSERT INTO ${Tables.LANGUAGES} (
  name, description, secret
) VALUES ($1, $2, $3)
  ON CONFLICT(name) DO UPDATE SET description=excluded.description`,
      [record.name, record.description, record.secret],
    ),
  );
};

export const createImmunity = async (name: string) => {
  return id(
    Tables.IMMUNITIES,
    await dbExecute(`INSERT INTO ${Tables.IMMUNITIES} (name) VALUES ($1)`, [name]),
  );
};

const createNameDescUnit = (table: TableName) => async (
  record: { name: string; description: string, unit: string | null },
) =>
  id(
    table,
    await dbExecute(
      `
INSERT INTO ${table} (
  name, description, unit
) VALUES ($1, $2, $3)`,
      [record.name, record.description, record.unit],
    ),
  );

export const createSpeedTrait = createNameDescUnit(Tables.SPEED_TRAITS);

export const createSense = createNameDescUnit(Tables.SENSES);
