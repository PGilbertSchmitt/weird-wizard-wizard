import { getTraditions } from '@/lib/db/magic';
import { cn } from '@/lib/utils';
import { useQuery } from '@tanstack/react-query';
import { TraditionCard } from './tradition-card';

export const Tome = () => {
  const results = useQuery({
    queryKey: ['traditions'],
    queryFn: getTraditions,
  });

  return (
    <>
      <h1>Magic Tome</h1>
      <p>All Traditions</p>
      {results.data && (
        <div
          className={cn(
            'flex flex-wrap justify-between gap-x-2 cursor-pointer mt-8',
          )}
        >
          {results.data.map((tradition) => (
            <TraditionCard key={tradition.id} tradition={tradition} />
          ))}
        </div>
      )}
    </>
  );
};
