import { useChoiceSelection } from '@/api/choice-selections';
import { SelectionItemProps } from '../character-selection';
import { Spinner } from '@/components/ui/neo/spinner';
import { useMemo } from 'react';
import { cn } from '@/lib/utils';

export const ChoiceTableSelection = ({ choices }: SelectionItemProps) => {
  const selectionIds = useMemo(
    () => choices.map((ch) => parseInt(ch.selection)),
    [choices],
  );

  return (
    <ul>
      {selectionIds.map((id, i) => (
        <li key={i}>
          <ChoiceSelectionData id={id} />
        </li>
      ))}
    </ul>
  );
};

const ChoiceSelectionData = ({ id }: { id: number }) => {
  const { data: selection } = useChoiceSelection(id);

  if (!selection) {
    return <Spinner />;
  }

  const { label, description } = selection;

  return (
    <>
      {label && (
        <p>
          <b>{label}</b>
        </p>
      )}
      <p className={cn('pl-2')}>{description}</p>
    </>
  );
};
