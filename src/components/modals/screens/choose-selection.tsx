import { cn, nth } from '@/lib/utils';
import { ModalChoiceProps } from '../switch';
import { useChoiceTable } from '@/api/choice-selections';
import { Button } from '@/components/ui/button';
import { useSaveChoice } from '@/api/characters';
import { useModal } from '@/hooks/modal';
import { useMemo, useState } from 'react';
import { FullChoice } from '@/types/choice_selections';

interface ChooseSelectProps extends ModalChoiceProps {
  selectionName: string;
}

export const ChooseSelection = ({
  character,
  keys,
  modifier,
  selectionName,
}: ChooseSelectProps) => {
  const { data: choiceTable } = useChoiceTable(selectionName);
  const { mutateAsync: saveChoice } = useSaveChoice(character.id);
  const { popNonErrorModal, pushError } = useModal();

  const [selected, setSelected] = useState<Array<FullChoice>>([]);

  const priorSelections = useMemo(() => {
    const relevantPriorChoices = character.selected_choices.filter(
      ([modifier]) => {
        const target = modifier.mod_details.target;

        if (target.type !== 'Choose') {
          return false;
        }

        if (target.data[0].type !== 'Select') {
          return false;
        }

        return target.data[0].data[1] === selectionName;
      },
    );

    return relevantPriorChoices
      .flatMap(([_, choices]) => choices.map((ch) => ch.id))
      .concat(selected.map((ch) => ch.id));
  }, [selectionName, character, selected]);

  if (!choiceTable) {
    return null;
  }

  const idxStr = keys.length === 1 ? '' : nth(0);
  const pickChoice = selected.length < keys.length;

  return pickChoice ? (
    <div className={cn('w-fit')}>
      <h1>
        Pick {idxStr} option from {selectionName}
      </h1>

      <div>
        {choiceTable.choices.map((choice) => (
          <Button
            key={choice.id}
            className={cn('w-full my-2')}
            disabled={priorSelections.includes(choice.id)}
            onClick={() => {
              setSelected([...selected, choice]);
            }}
          >
            <h3>{choice.label}</h3>
            <p>{choice.description}</p>
          </Button>
        ))}
      </div>
    </div>
  ) : (
    <div className={cn('w-80')}>
      <h3 className="mb-4">Selections:</h3>

      <ul>
        {selected.map((choice) => (
          <li key={choice.id}>- {choice.label}</li>
        ))}
      </ul>

      <Button
        className={cn('w-full mt-4')}
        onClick={() => {
          saveChoice({
            modifier,
            values: selected.map((choice) => choice.id.toString()),
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
    </div>
  );
};
