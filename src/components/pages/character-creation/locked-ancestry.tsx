import { useAncestry } from '@/api/ancestries';
import { StaticCard } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { calculateAncestryAttributes } from '../catalogue/ancestries';
import { TalentCard } from '../catalogue/talent-card';
import { useEffect } from 'react';

interface LockedAncestryProps {
  ancestryId: number;
  setName: (ancestryName: string) => void;
}

export const LockedAncestry = ({
  ancestryId,
  setName,
}: LockedAncestryProps) => {
  const { data: ancestry } = useAncestry(ancestryId);

  useEffect(() => {
    if (ancestry?.name) {
      setName(ancestry.name);
    }
  }, [ancestry?.name]);

  if (!ancestry) {
    return null;
  }

  const attributes = calculateAncestryAttributes(ancestry);

  return (
    <div className={cn('w-dvw max-w-250 px-4')}>
      <StaticCard className={cn('my-4 p-0')}>
        <div className="flex flex-row gap-4 items-center p-2">
          <h2>{ancestry.name}</h2>
          <p>(determined by starting path)</p>
        </div>

        <Separator />

        <div className={cn('bg-secondary-background text-foreground')}>
          <div className={cn('p-4 flex flex-row flex-wrap gap-x-5 gap-y-1')}>
            {attributes.map((attr) => (
              <p className="block">
                <b>{attr.label}:</b> {attr.value}
              </p>
            ))}
          </div>

          {ancestry.talents.length > 0 && (
            <>
              <Separator />
              <div className={cn('py-1 px-4')}>
                {ancestry.talents.map((talent) => (
                  <TalentCard key={talent.id} talent={talent} />
                ))}
              </div>
            </>
          )}
        </div>
      </StaticCard>
    </div>
  );
};
