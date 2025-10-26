import {
  NonNovicePathRecord,
  NovicePathRecord,
} from '../import-data/path-import';
import { dbExecute, id, Tables } from './client';
import { PathKind, PathKinds } from './enums';

export const createNovicePath = async (
  record: NovicePathRecord,
  ancestryId?: number,
) => {
  return id(
    Tables.PATHS,
    await dbExecute(
      `
INSERT INTO ${Tables.PATHS} (
  name,
  kind,
  category,
  description,
  req_str,
  req_agl,
  req_int,
  req_will,
  ancestry
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`,
      [
        record.name,
        PathKinds.NOVICE,
        record.origin_locked ? 'Ancestry' : 'Archtype',
        record.description,
        record.init_scores_lvl_1.strength,
        record.init_scores_lvl_1.agility,
        record.init_scores_lvl_1.intellect,
        record.init_scores_lvl_1.will,
        ancestryId,
      ],
    ),
  );
};

export const createNonNovicePath = async (
  record: NonNovicePathRecord,
  pathKind: PathKind,
) => {
  return id(
    Tables.PATHS,
    await dbExecute(
      `
INSERT INTO ${Tables.PATHS} (
  name, kind, category, description
) VALUES ($1, $2, $3, $4)`,
      [record.name, pathKind, record.sub_path, record.description],
    ),
  );
};
