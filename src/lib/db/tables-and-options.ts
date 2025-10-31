import { InfoTable, OptionBlock } from '../types';
import { dbExecute, dbSelect, id, Tables } from './client';
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
      `INSERT INTO ${Tables.INFO_TABLES} (
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

interface TableRow {
  id: number;
  name: string;
  kind: TableType;
  key_label: string;
  value_label: string;
  key: string;
  value: string;
}

export const getInfoTables = async (ids: number[]) => {
  const tableRows = await dbSelect<TableRow>(
    `SELECT
      it.id,
      it.name,
      it.kind,
      it.key_label,
      it.value_label,
      itr.key,
      itr.value
    FROM info_tables it
    JOIN info_table_rows itr ON it.id=itr.info_table_id
    WHERE it.id IN ${paramPairs(ids.length, 1)}`,
    ids,
  );

  const tableMap = new Map<number, InfoTable>();
  for (const row of tableRows) {
    const table = tableMap.get(row.id);
    if (table) {
      table.rows.push({ key: row.key, value: row.value });
    } else {
      tableMap.set(row.id, {
        id: row.id,
        name: row.name,
        kind: row.kind,
        keyLabel: row.key_label,
        valueLabel: row.value_label,
        rows: [{ key: row.key, value: row.value }],
      });
    }
  }

  return tableMap;
};

interface OptionBlockRow {
  id: number;
  name: string;
  value: string;
}

export const getOptionBlocks = async (ids: number[]) => {
  const optionRows = await dbSelect<OptionBlockRow>(
    `SELECT ob.id, ob.name, obr.value
    FROM option_blocks ob
    JOIN option_block_rows obr ON ob.id=obr.option_block_id
    WHERE ob.id IN ${paramPairs(ids.length, 1)}`,
    ids,
  );

  const optionMap = new Map<number, OptionBlock>();
  for (const row of optionRows) {
    const block = optionMap.get(row.id);
    if (block) {
      block.values.push(row.value);
    } else {
      optionMap.set(row.id, {
        id: row.id,
        name: row.name,
        values: [row.value],
      });
    }
  }

  return optionMap;
};
