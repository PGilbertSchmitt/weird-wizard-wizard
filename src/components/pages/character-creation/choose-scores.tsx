import { useFullNovicePath } from '@/api/paths';
import { Button } from '@/components/ui/button';
import { StaticCard } from '@/components/ui/card';
import { Paragraph } from '@/components/ui/paragraph';
import { ScoreForm, Scores } from '@/components/ui/score-form';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { max, min } from 'ramda';
import { useEffect, useMemo, useState } from 'react';

interface ChooseScoresProps {
  novicePathId: number;
  onConfirm: (scores: Scores) => void;
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

  const pointsRemaining = highestSumScore - currentSumScore;
  const minScores: Scores = {
    strength: 1,
    agility: 1,
    intellect: 1,
    will: 1,
  };
  const maxScores: Scores = {
    strength: min(highestSingleScore, scores.strength + pointsRemaining),
    agility: min(highestSingleScore, scores.agility + pointsRemaining),
    intellect: min(highestSingleScore, scores.intellect + pointsRemaining),
    will: min(highestSingleScore, scores.will + pointsRemaining),
  };

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
            'bg-secondary-background text-foreground rounded-b-base flex flex-col items-center',
          )}
        >
          <div className={cn('px-6 py-2')}>
            <p>Points remaining: {pointsRemaining}</p>
          </div>

          <div className={cn('flex flex-row')}>
            <ScoreForm
              scores={scores}
              setScores={setScores}
              minScores={minScores}
              maxScores={maxScores}
            />
          </div>

          <Button className={cn('p-2 mb-4')} onClick={() => onConfirm(scores)}>
            Confirm
          </Button>
        </div>
      </StaticCard>
    </div>
  );
};
