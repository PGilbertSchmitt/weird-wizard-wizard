import {
  ProfessionCategoryRecord,
  ProfessionRecord,
} from '../import-data/profession-import';
import { dbExecute, id, nameAndDescQuery } from './client';

export const createProfessionCategory = async (
  record: ProfessionCategoryRecord,
) => {
  return await nameAndDescQuery('profession_categories', record);
};

export const createProfession = async (record: ProfessionRecord) => {
  return id(
    'professions',
    await dbExecute(
      `
INSERT INTO professions (
  name, description, category
) VALUES ($1, $2, $3)`,
      [record.name, record.description, record.category],
    ),
  );
};
