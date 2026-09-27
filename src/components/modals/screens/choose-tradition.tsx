import { StaticCard } from '@/components/ui/card';
import { cn } from '@/lib/utils';
import { TraditionIndexItem } from '@/types/magic';

interface ChooseTraditionProps {
  idxStr: string;
  traditions: Array<TraditionIndexItem>;
  onSelect: (traditionId: number, traditionName: string) => void;
}

export const ChooseTradition = ({
  onSelect,
  traditions,
  idxStr,
}: ChooseTraditionProps) => {
  return (
    <div>
      <h1>Pick {idxStr} tradition</h1>

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
