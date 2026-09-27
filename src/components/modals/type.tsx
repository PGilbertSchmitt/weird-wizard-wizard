import {
  ChooseTarget,
  Condition,
  ModifierPathNode,
  WhenMod,
} from '@/types/modifiers';

export const ModalTypes = {
  CHOOSE: 'Choose',
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

// For self contained decisions made by the player
export interface CharacterChoiceModalData {
  type: typeof ModalTypes.CHOOSE;
  characterId: number;
  modifier: FullChoiceModifier;
  source: string;
}

// For errors during the import process
export interface ErrorModalData {
  type: typeof ModalTypes.ERROR;
  error: React.ReactNode;
}

// Might have non-CharacterChoice-based modals
export type ModalData = CharacterChoiceModalData | ErrorModalData;
