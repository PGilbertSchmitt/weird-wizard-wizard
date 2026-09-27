import { cn, nth, ValueOf } from '@/lib/utils';
import { ModalChoiceProps } from '../../switch';
import { useReducer } from 'react';
import { clone, isNil } from 'ramda';
import { Button } from '@/components/ui/button';
import { useSaveChoice } from '@/api/characters';
import { ChooseItem } from './choose-item';
import { useModal } from '@/hooks/modal';
import { WithTraditions } from '@/components/providers/with-traditions';
import { ChooseTradition } from '../choose-tradition';

export const ChooseTraditionFlow = ({
  character,
  modifier,
  keys,
}: ModalChoiceProps) => {
  const { popNonErrorModal, pushError } = useModal();
  const { mutateAsync: saveChoice } = useSaveChoice(character.id);
  const [state, dispatch] = useReducer<FormState, [FormAction]>(
    reducer,
    keys.map((key) => ({ key })),
  );
  const [curStep, curIdx] = getStepAndId(state);

  const idxStr = nth(curIdx);
  const selectedTalentIds = state
    .filter((choice) => choice.kind === 'talent')
    .map((choice) => choice.itemId!);

  return (
    <div className={cn('w-fit flex flex-col items-center gap-4')}>
      {curStep === ItemSteps.PICK_TRADITION && (
        <WithTraditions
          onRender={(traditions) => (
            <ChooseTradition
              idxStr={idxStr}
              traditions={traditions}
              onSelect={(id, name) => {
                dispatch({
                  type: FormActions.SET_TRADITION,
                  idx: curIdx,
                  data: { id, name },
                });
              }}
            />
          )}
        />
      )}

      {curStep === ItemSteps.PICK_ITEM && (
        <ChooseItem
          character={character}
          idxStr={idxStr}
          traditionId={state[curIdx].traditionId!}
          selectedTalentIds={selectedTalentIds}
          onSelect={(id, name, kind) => {
            dispatch({
              type: FormActions.SET_ITEM,
              idx: curIdx,
              data: { id, name, kind },
            });
          }}
        />
      )}

      {curStep === ItemSteps.READY && (
        <>
          <h3>Selections:</h3>
          {state.map((choice) => {
            return (
              <div key={choice.key}>
                <p>
                  <b>{choice.traditionName}</b> tradition and the{' '}
                  <b>{choice.itemName}</b> {choice.kind}
                </p>
              </div>
            );
          })}
          <Button
            onClick={() => {
              saveChoice({
                modifier,
                values: state.map(
                  (choice) =>
                    `${choice.traditionId}|${choice.kind === 'talent' ? 't' : 's'}|${choice.itemId}`,
                ),
              })
                .then(popNonErrorModal)
                .catch((err) => {
                  popNonErrorModal();
                  pushError(err.toString());
                });
            }}
          >
            Confirm
          </Button>
        </>
      )}
    </div>
  );
};

export interface ItemChoice {
  key: string;
  traditionId?: number;
  traditionName?: string;
  kind?: 'talent' | 'spell';
  itemId?: number;
  itemName?: string;
}

const ItemSteps = {
  PICK_TRADITION: 'PICK_TRADITION',
  PICK_ITEM: 'PICK_ITEM',
  READY: 'READY',
} as const;
type ItemStep = ValueOf<typeof ItemSteps>;

const itemStep = (itemChoice: ItemChoice): ItemStep => {
  switch (true) {
    case isNil(itemChoice.traditionId):
      return ItemSteps.PICK_TRADITION;
    case isNil(itemChoice.kind) || isNil(itemChoice.itemId):
      return ItemSteps.PICK_ITEM;
    default:
      return ItemSteps.READY;
  }
};

const getStepAndId = (choices: FormState): [ItemStep, number] => {
  for (let [idx, choice] of choices.entries()) {
    const curStep = itemStep(choice);
    if (curStep !== ItemSteps.READY) {
      return [curStep, idx];
    }
  }

  return [ItemSteps.READY, -1];
};

type FormState = Array<ItemChoice>;

const FormActions = {
  SET_TRADITION: 'SET_TRADITION',
  SET_ITEM: 'SET_ITEM',
} as const;

type FormAction =
  | {
      type: typeof FormActions.SET_TRADITION;
      idx: number;
      data: { id: number; name: string };
    }
  | {
      type: typeof FormActions.SET_ITEM;
      idx: number;
      data: { id: number; name: string; kind: 'talent' | 'spell' };
    };

const reducer = (state: FormState, action: FormAction): FormState => {
  const newState = clone(state);
  switch (action.type) {
    case 'SET_TRADITION': {
      const stateItem = newState[action.idx];
      stateItem.traditionId = action.data.id;
      stateItem.traditionName = action.data.name;

      return newState;
    }
    case 'SET_ITEM': {
      const stateItem = newState[action.idx];
      stateItem.kind = action.data.kind;
      stateItem.itemId = action.data.id;
      stateItem.itemName = action.data.name;

      return newState;
    }
  }
};
