import {
  ProfessionCategoryRecord,
  ProfessionRecord,
} from '../import-data/profession-import';
import { dbExecute, id, nameAndDescQuery, Tables } from './client';

export const createProfessionCategory = async (
  record: ProfessionCategoryRecord,
) => {
  return await nameAndDescQuery(Tables.PROFESSION_CATEGORIES, record);
};

export const createProfession = async (record: ProfessionRecord) => {
  return id(
    Tables.PROFESSIONS,
    await dbExecute(
      `
INSERT INTO professions (
  name, description, category
) VALUES ($1, $2, $3)`,
      [record.name, record.description, record.category],
    ),
  );
};
