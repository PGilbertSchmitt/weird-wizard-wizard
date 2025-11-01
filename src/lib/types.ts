import {
  MagicTalentCharge,
  MagicTalentRestoration,
  PathKind,
  TableType,
} from './db/enums';

// Maybe one day, this will be a built-in type
export type ValueOf<T> = T[keyof T];

/* DB Selector Types */

export interface TraditionIndexItem {
  id: number;
  name: string;
  blurb: string;
  description: string;
}

export interface FullTradition {
  id: number;
  name: string;
  blurb: string;
  description: string;
  specialInfo: string | null;
  infoTable: InfoTable | null;
  talents: Array<TalentItem>;
  noviceSpells: Array<SpellItem>;
  expertSpells: Array<SpellItem>;
  masterSpells: Array<SpellItem>;
}

export interface InfoTable {
  id: number;
  name: string;
  kind: TableType;
  keyLabel: string;
  valueLabel: string;
  rows: Array<{ key: string; value: string }>;
}

export interface OptionBlock {
  id: number;
  name: string;
  values: string[];
}

export interface SpellItem {
  id: number;
  name: string;
  description: string;
  pathKind: PathKind;
  castings: number;
  duration: string;
  target: string;
  condition: string | null;
  ritual: boolean;
  infoTable: InfoTable | null;
  optionBlock: OptionBlock | null;
}

export interface TalentItem {
  id: number;
  name: string;
  description: string;
  magical: boolean;
  charges: MagicTalentCharge;
  restore: MagicTalentRestoration;
  infoTable: InfoTable | null;
  optionBlock: OptionBlock | null;
  activations: string[];
}
