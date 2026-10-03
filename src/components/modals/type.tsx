import { FullCharacter } from '@/types/character';
import {
  ChooseTarget,
  Condition,
  ModifierPathNode,
  WhenMod,
} from '@/types/modifiers';

export const ModalTypes = {
  CHOOSE: 'Choose',
  CHOOSE_PATH: 'Choose Path',
  ERROR: 'Error',
} as const;
// type ModalType = typeof ModalTypes[keyof typeof ModalTypes];

// All the data of a FullModifier with a Choice target.
// Just makes it easier to work with on the TS side.
export interface FullChoiceModifier {
  path_node: ModifierPathNode;
  mod_details: {
    when: WhenMod;
    target: {
      type: 'Choose';
      data: [ChooseTarget, Array<string>];
    };
    condition: Condition;
  };
}

// For self contained decisions made by the player mased on modifiers
export interface CharacterChoiceModalData {
  type: typeof ModalTypes.CHOOSE;
  character: FullCharacter;
  modifier: FullChoiceModifier;
  source: string;
}

// For path selection (which don't rely on modifiers)
export interface CharacterPathModalData {
  type: typeof ModalTypes.CHOOSE_PATH;
  character: FullCharacter;
  kind: 'Expert' | 'Master';
}

// For errors during the import process
export interface ErrorModalData {
  type: typeof ModalTypes.ERROR;
  error: React.ReactNode;
}

// Might have non-CharacterChoice-based modals
export type ModalData =
  CharacterChoiceModalData | CharacterPathModalData | ErrorModalData;
