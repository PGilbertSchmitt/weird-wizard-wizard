import { useFullNovicePath } from '@/api/paths';
import { Button } from '@/components/ui/button';
import { StaticCard } from '@/components/ui/card';
import { ControlledCounter } from '@/components/ui/controlled-counter';
import { Paragraph } from '@/components/ui/paragraph';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { max, toPairs } from 'ramda';
import { useEffect, useMemo, useState } from 'react';

interface ChooseScoresProps {
  novicePathId: number;
  onConfirm: (scores: Scores) => void;
}

// type FourTuple = [number, number, number, number];
export interface Scores {
  strength: number;
  agility: number;
  intellect: number;
  will: number;
}

export const ChooseScores = ({
  novicePathId,
  onConfirm,
}: ChooseScoresProps) => {
  const { data: novicePath } = useFullNovicePath(novicePathId);

  const [scores, setScores] = useState<null | Scores>(null);

  useEffect(() => {
    if (novicePath) {
      setScores({
        strength: novicePath.rec_str,
        agility: novicePath.rec_agl,
        intellect: novicePath.rec_int,
        will: novicePath.rec_will,
      });
    }
  }, [novicePath]);

  const highestSingleScore = useMemo(() => {
    if (novicePath) {
      return [
        novicePath.rec_str,
        novicePath.rec_agl,
        novicePath.rec_int,
        novicePath.rec_will,
      ].reduce(max, 14);
    } else {
      return 14;
    }
  }, [novicePath]);

  if (!novicePath || !scores) {
    return null;
  }

  const highestSumScore =
    novicePath.rec_str +
    novicePath.rec_agl +
    novicePath.rec_int +
    novicePath.rec_will;

  const currentSumScore =
    scores.strength + scores.agility + scores.intellect + scores.will;

  return (
    <div className="flex flex-col align-middle w-150">
      <StaticCard className={cn('my-4 p-0')}>
        <div
          className={cn(
            'flex flex-row justify-center items-center p-2 cursor-pointer',
          )}
        >
          <h2>Starting Ability Scores</h2>
        </div>

        <Separator />

        <div className={cn('bg-secondary-background text-foreground')}>
          <div className={cn('px-4 py-1')}>
            <Paragraph>
              Set your base ability scores. If your novice path allows you pick
              bonuses to your abilities, you will get the opportunity to pick
              those later. The initial values are recommended by your Novice
              path, but you're free to do as you please because you are a
              strong, brave woman.
            </Paragraph>
          </div>
        </div>

        <Separator />

        <div
          className={cn(
            'bg-secondary-background text-foreground flex flex-col items-center',
          )}
        >
          <div className={cn('px-6 py-2')}>
            <p>Points remaining: {highestSumScore - currentSumScore}</p>
          </div>

          <div className={cn('flex flex-row')}>
            {toPairs(scores).map(([ability, value]) => {
              return (
                <div
                  key={ability}
                  className={cn('flex flex-col items-center m-4')}
                >
                  {fixCase(ability)}
                  <ControlledCounter
                    value={value}
                    onAdd={() =>
                      setScores({
                        ...scores,
                        [ability]: value + 1,
                      })
                    }
                    onSubtract={() =>
                      setScores({
                        ...scores,
                        [ability]: value - 1,
                      })
                    }
                    subtractDisabled={value <= 1}
                    addDisabled={
                      value >= highestSingleScore ||
                      currentSumScore >= highestSumScore
                    }
                  />
                </div>
              );
            })}
          </div>

          <Button className={cn('p-2 mb-4')} onClick={() => onConfirm(scores)}>
            Confirm
          </Button>
        </div>
      </StaticCard>
    </div>
  );
};

const fixCase = (s: string) => `${s[0].toUpperCase()}${s.slice(1)}`;
