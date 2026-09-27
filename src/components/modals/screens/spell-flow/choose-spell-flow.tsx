import { useModal } from '@/hooks/modal';
import { ModalChoiceProps } from '../../switch';
import { useSaveChoice } from '@/api/characters';
import { useMemo, useReducer } from 'react';
import { cn, nth, ValueOf } from '@/lib/utils';
import { clone, isNil, sortBy, uniqBy } from 'ramda';
import { ChooseTradition } from '../choose-tradition';
import { ChooseSpells } from '../choose-spells';
import { PathKind } from '@/types/etc';
import { Button } from '@/components/ui/button';
import { TraditionIndexItem } from '@/types/magic';

interface ChooseSpellFlowProps extends ModalChoiceProps {
  maxKind: PathKind;
}

export const ChooseSpellFlow = ({
  character,
  modifier,
  maxKind,
  keys,
}: ChooseSpellFlowProps) => {
  const { popNonErrorModal, pushError } = useModal();
  const { mutateAsync: saveChoice } = useSaveChoice(character.id);
  const [state, dispatch] = useReducer<FormState, [FormAction]>(
    reducer,
    keys.map((key) => ({ key })),
  );

  const [curStep, curIdx] = getStepAndId(state);
  const idxStr = nth(curIdx);
  const availableTraditions = useMemo(() => {
    const traditions = character.traditions.map((t) => t[0]);
    return sortBy(
      (t: TraditionIndexItem) => t.id,
      uniqBy((t) => t.id, traditions),
    );
  }, [character.traditions]);

  return (
    <div className={cn('w-fit flex flex-col items-center gap-4')}>
      {curStep === SpellSteps.PICK_TRADITION && (
        <ChooseTradition
          idxStr={idxStr}
          traditions={availableTraditions}
          onSelect={(id, name) => {
            dispatch({
              type: FormActions.SET_TRADITION,
              idx: curIdx,
              data: { id, name },
            });
          }}
        />
      )}

      {curStep === SpellSteps.PICK_SPELL && (
        <ChooseSpells
          traditionId={state[curIdx].traditionId!}
          maxKind={maxKind}
          onSelect={(id, name) =>
            dispatch({
              type: FormActions.SET_SPELL,
              idx: curIdx,
              data: { id, name },
            })
          }
        />
      )}

      {curStep === SpellSteps.READY && (
        <>
          <h3>Selections:</h3>
          {state.map((choice) => {
            return (
              <div key={choice.key}>
                <p>
                  <b>{choice.spellName}</b> spell from the{' '}
                  <b>{choice.traditionName}</b> tradition
                </p>
              </div>
            );
          })}
          <Button
            onClick={() => {
              saveChoice({
                modifier,
                values: state.map((choice) => choice.spellId!.toString()),
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

interface SpellChoice {
  key: string;
  traditionId?: number;
  traditionName?: string;
  spellId?: number;
  spellName?: string;
}

const SpellSteps = {
  PICK_TRADITION: 'PICK_TRADITION',
  PICK_SPELL: 'PICK_SPELL',
  READY: 'READY',
} as const;
type SpellStep = ValueOf<typeof SpellSteps>;

const spellStep = (spellChoice: SpellChoice): SpellStep => {
  switch (true) {
    case isNil(spellChoice.traditionId):
      return SpellSteps.PICK_TRADITION;
    case isNil(spellChoice.spellId):
      return SpellSteps.PICK_SPELL;
    default:
      return SpellSteps.READY;
  }
};

const getStepAndId = (choices: FormState): [SpellStep, number] => {
  for (let [idx, choice] of choices.entries()) {
    const curStep = spellStep(choice);
    if (curStep !== SpellSteps.READY) {
      return [curStep, idx];
    }
  }

  return [SpellSteps.READY, -1];
};

type FormState = Array<SpellChoice>;

const FormActions = {
  SET_TRADITION: 'SET_TRADITION',
  SET_SPELL: 'SET_SPELL',
} as const;

type FormAction =
  | {
      type: typeof FormActions.SET_TRADITION;
      idx: number;
      data: { id: number; name: string };
    }
  | {
      type: typeof FormActions.SET_SPELL;
      idx: number;
      data: { id: number; name: string };
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
    case 'SET_SPELL': {
      const stateItem = newState[action.idx];
      stateItem.spellId = action.data.id;
      stateItem.spellName = action.data.name;
      return newState;
    }
  }
};
