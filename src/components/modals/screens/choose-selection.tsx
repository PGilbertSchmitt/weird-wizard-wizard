import { cn, nth } from '@/lib/utils';
import { ModalChoiceProps } from '../switch';
import { useChoiceTable } from '@/api/choice-selections';
import { Button } from '@/components/ui/button';
import { useMemo, useState } from 'react';
import { FullChoice } from '@/types/choice_selections';
import { ConfirmSelection } from './confirm-selection';

interface ChooseSelectProps extends ModalChoiceProps {
  selectionName: string;
}

export const ChooseSelection = ({
  character,
  keyCount,
  modifier,
  selectionName,
}: ChooseSelectProps) => {
  const { data: choiceTable } = useChoiceTable(selectionName);

  const [selected, setSelected] = useState<Array<FullChoice>>([]);

  const priorSelections = useMemo(() => {
    const relevantPriorChoices = character.selected_choices.filter(
      ([modifier]) => {
        const target = modifier.mod_details.target;

        if (target.type !== 'Choose') {
          return false;
        }

        if (target.data.type !== 'Select') {
          return false;
        }

        return target.data.data[1] === selectionName;
      },
    );

    return relevantPriorChoices
      .flatMap(([_, choices]) => choices.map((ch) => parseInt(ch.selection)))
      .concat(selected.map((ch) => ch.id));
  }, [selectionName, character, selected]);

  if (!choiceTable) {
    return null;
  }

  const idxStr = keyCount === 1 ? '' : nth(0);
  const pickChoice = selected.length < keyCount;

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
    <ConfirmSelection
      characterId={character.id}
      modifier={modifier}
      values={selected.map((choice) => choice.id.toString())}
    >
      <ul>
        {selected.map((choice) => (
          <li key={choice.id}>- {choice.label}</li>
        ))}
      </ul>
    </ConfirmSelection>
  );
};
