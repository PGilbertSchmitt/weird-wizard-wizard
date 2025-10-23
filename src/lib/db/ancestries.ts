import { AncestryRecord } from '../import-data/ancestry-import';
import { dbExecute, id } from './client';

export const createAncestry = async (record: AncestryRecord) => {
  return id(
    'ancestries',
    await dbExecute(
      `
INSERT INTO ancestries (
  name,
  descriptor,
  size,
  speed,
  add_health,
  add_nat_def
) VALUES ($1, $2, $3, $4, $5, $6)
  `,
      [
        record.ancestry,
        record.descriptor || null,
        record.size,
        record.speed,
        record.add_health,
        record.add_health,
      ],
    ),
  );
};
