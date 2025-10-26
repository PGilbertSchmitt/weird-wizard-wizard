import { LevelRecord } from '../import-data/level-import';
import { dbExecute, id, Tables } from './client';

export const createLevel = async (record: LevelRecord, pathId: number) => {
  return id(
    Tables.LEVELS,
    await dbExecute(
      `
INSERT INTO ${Tables.LEVELS} (
  path,
  level,
  add_health,
  add_nat_def,
  add_arm_def,
  add_bonus_dmg,
  add_speed,
  size
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`,
      [
        pathId,
        record.level,
        record.health,
        record.nat_def,
        record.armed_def,
        record.bonus_dmg,
        record.speed,
        record.size,
      ],
    ),
  );
};
