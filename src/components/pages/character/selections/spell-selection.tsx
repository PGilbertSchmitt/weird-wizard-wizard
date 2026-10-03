import { useMemo } from 'react';
import { SelectionItemProps } from '../character-selection';
import { SpellName } from './tradition-selection';

export const SpellSelection = ({ choices }: SelectionItemProps) => {
  const spellIds = useMemo(
    () => choices.map((ch) => parseInt(ch.selection)),
    [choices],
  );

  return (
    <ul>
      {spellIds.map((id, i) => (
        <li key={i}>
          + <SpellName id={id} />
        </li>
      ))}
    </ul>
  );
};
