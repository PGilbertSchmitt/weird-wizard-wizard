import { useTraditions } from '@/api/magic';
import { NavCard } from '@/components/ui/card';
import { cardStyle, pressStyle } from '@/components/ui/styles';
import { cn } from '@/lib/utils';

export const Tome = () => {
  const { isFetched, data: traditions } = useTraditions();

  if (!isFetched) {
    return null;
  }

  return (
    <div>
      <h1>Traditions</h1>

      <div className={cn('w-full max-w-400 mx-2 grid grid-cols-3')}>
        {traditions?.map((tradition) => (
          <NavCard
            className={cn(cardStyle, pressStyle, 'm-2')}
            href={`/tome/${tradition.id}`}
          >
            <h2>{tradition.name}</h2>
            <p>{tradition.blurb}</p>
          </NavCard>
        ))}
      </div>
    </div>
  );
};
