import { SelectionItemProps } from '../character-selection';

export const ScoreSelection = ({ choices }: SelectionItemProps) => (
  <ul>
    {choices.map((ch) => (
      <li>+1 <b>{ch.selection}</b></li>
    ))}
  </ul>
);
