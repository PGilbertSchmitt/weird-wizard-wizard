import { StaticCard } from '@/components/ui/card';
import { Spinner } from '@/components/ui/neo/spinner';
import { cn } from '@/lib/utils';
import { TraditionIndexItem } from '@/types/magic';
import { useEffect } from 'react';

interface ChooseTraditionProps {
  idxStr: string;
  traditions: Array<TraditionIndexItem>;
  forSpells?: boolean;
  onSelect: (traditionId: number, traditionName: string) => void;
}

export const ChooseTradition = ({
  onSelect,
  traditions,
  idxStr,
  forSpells = false,
}: ChooseTraditionProps) => {
  useEffect(() => {
    if (traditions.length === 1) {
      const tradition = traditions[0];
      // This callback should be idempotent, so even if the effect is called twice,
      // it should be fine.
      onSelect(tradition.id, tradition.name);
    }
  }, [traditions]);

  if (traditions.length === 1) {
    return <Spinner />;
  }

  return (
    <div>
      <h1>
        {forSpells
          ? `Pick tradition for ${idxStr} spell`
          : `Pick ${idxStr} tradition`}
      </h1>

      <div className={cn('w-full max-w-400 mx-2 grid grid-cols-3')}>
        {traditions.map((tradition) => (
          <StaticCard
            className="m-2"
            key={tradition.id}
            onClick={() => onSelect(tradition.id, tradition.name)}
          >
            <h2>{tradition.name}</h2>
            <p>{tradition.blurb}</p>
          </StaticCard>
        ))}
      </div>
    </div>
  );
};
