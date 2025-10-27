// It's called "enums.ts", but you'll never catch me using a real TS enum
// (outside of work)

import { ValueOf } from '../types';

export const Sizes = {
  SMALL: 'sm',
  MEDIUM: 'md',
  LARGE: 'lg',
} as const;

export type Size = ValueOf<typeof Sizes>;

export const Units = {
  INCHES: 'inches',
  YARDS: 'yards',
} as const;

export type Unit = ValueOf<typeof Units>;

export const PathKinds = {
  NOVICE: 'Novice',
  EXPERT: 'Expert',
  MASTER: 'Master',
} as const;

export type PathKind = ValueOf<typeof PathKinds>;

export const MagicTalentCharges = {
  NONE: 'None',
  ONE: 'One',
  // One at level one, two at level 3, and 3 at level 7
  ONE_TWO_THREE: 'OneTwoThree',
} as const;

export type MagicTalentCharge = ValueOf<typeof MagicTalentCharges>;

export const MagicTalentRestorations = {
  NONE: 'None',
  LUCK_ENDS: 'Luck Ends',
  REST: 'Rest',
  DAY: 'Day',
  HOUR: 'Hour',
  MINUTE: 'Minute',
  START_OF_NEXT_TURN: 'Start Of Next Turn',
  END_OF_NEXT_TURN: 'End Of Next Turn',
} as const;

export type MagicTalentRestoration = ValueOf<typeof MagicTalentRestorations>;

export const TableTypes = {
  TABLE: 'TABLE',
  BLOCK: 'BLOCK',
  ROLL: 'ROLL',
} as const;

export type TableType = ValueOf<typeof TableTypes>;
