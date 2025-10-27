import { dbExecute, id, Tables } from './client';
import { paramPairs } from './db-utils';
import { TableType } from './enums';

export const createOptionBlock = async (label: string) => {
  return id(
    Tables.OPTION_BLOCKS,
    await dbExecute(`INSERT INTO ${Tables.OPTION_BLOCKS} (name) VALUES ($1)`, [
      label,
    ]),
  );
};

export const createOptionBlockRows = async (
  optionBlockId: number,
  options: string[],
) => {
  const values = options.reduce((acc: Array<string | number>, opt) => {
    acc.push(optionBlockId, opt);
    return acc;
  }, []);

  await dbExecute(
    `INSERT INTO ${Tables.OPTION_BLOCK_ROWS} (option_block_id, value) VALUES ${paramPairs(2, options.length)}`,
    values,
  );
};

export const createInfoTable = async (
  name: string,
  kind: TableType,
  keyLabel: string,
  valueLabel: string,
) =>
  id(
    Tables.INFO_TABLES,
    await dbExecute(
      `
INSERT INTO ${Tables.INFO_TABLES} (
  name, kind, key_label, value_label
) VALUES ($1, $2, $3, $4)`,
      [name, kind, keyLabel, valueLabel],
    ),
  );

export const createInfoTableRows = async (
  infoTableId: number,
  rows: Array<{ key: string; value: string }>,
) => {
  const values = rows.reduce((acc: Array<string | number>, { key, value }) => {
    acc.push(infoTableId, key, value);
    return acc;
  }, []);
  await dbExecute(
    `INSERT INTO ${Tables.INFO_TABLE_ROWS} (info_table_id, key, value) VALUES ${paramPairs(3, rows.length)}`,
    values,
  );
};
