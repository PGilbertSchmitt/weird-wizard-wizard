import { useMemo } from 'react';
import { SelectionItemProps } from '../character-selection';
import {
  useMagicTalentName,
  useSpellName,
  useTraditionName,
} from '@/api/magic';
import { Spinner } from '@/components/ui/neo/spinner';

interface Selection {
  traditionId: number;
  kind: 't' | 's';
  kindId: number;
}

export const TraditionSelection = ({ choices }: SelectionItemProps) => {
  const selections = useMemo(
    () =>
      choices.map((ch): Selection => {
        const [tid, kind, kid] = ch.selection.split('|');
        return {
          traditionId: parseInt(tid),
          kind: kind === 't' ? 't' : 's',
          kindId: parseInt(kid),
        };
      }),
    [choices],
  );

  return (
    <ul>
      {selections.map(({ traditionId, kind, kindId }, i) =>
        kind === 't' ? (
          <li key={i}>
            + <TraditionName id={traditionId} /> Tradition with talent{' '}
            <MagicTalentName id={kindId} />
          </li>
        ) : (
          <li key={i}>
            + <TraditionName id={traditionId} /> Tradition with spell{' '}
            <SpellName id={kindId} />
          </li>
        ),
      )}
    </ul>
  );
};

interface IdProp {
  id: number;
}

const TraditionName = ({ id }: IdProp) => {
  const { data: name } = useTraditionName(id);
  if (!name) {
    return <Spinner />;
  }
  return <span><b>{name}</b></span>;
};

const MagicTalentName = ({ id }: IdProp) => {
  const { data: name } = useMagicTalentName(id);
  if (!name) {
    return <Spinner />;
  }
  return <span><b>{name}</b></span>;
};

export const SpellName = ({ id }: IdProp) => {
  const { data: name } = useSpellName(id);
  if (!name) {
    return <Spinner />;
  }
  return <span><b>{name}</b></span>;
};
