import {
  LanguageRecord,
  SenseRecord,
  SpeedTraitRecord,
} from '../import-data/natural-attribute-import';
import { dbExecute, id, nameAndDescQuery } from './client';

export const createLanguage = async (record: LanguageRecord) => {
  return id(
    'languages',
    await dbExecute(
      `
INSERT INTO languages (
  name, description, secret
) VALUES ($1, $2, $3)
  ON CONFLICT(name) DO UPDATE SET description=excluded.description`,
      [record.name, record.description, record.secret],
    ),
  );
};

export const createSpeedTrait = async (record: SpeedTraitRecord) => {
  return await nameAndDescQuery('speed_traits', record);
};

export const createSense = async (record: SenseRecord) => {
  return await nameAndDescQuery('senses', record);
};

export const createImmunity = async (name: string) => {
  return id(
    'immunities',
    await dbExecute(`INSERT INTO immunities (name) VALUES ($1)`, [name]),
  );
};
