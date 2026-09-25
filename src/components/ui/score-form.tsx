import { toPairs } from 'ramda';
import { ControlledCounter } from './controlled-counter';
import { cn } from '@/lib/utils';

export interface Scores {
  strength: number;
  agility: number;
  intellect: number;
  will: number;
}

interface ScoreFormProps {
  scores: Scores;
  minScores: Scores;
  maxScores: Scores;
  setScores: (newScores: Scores) => void;
}

export const ScoreForm = ({
  scores,
  setScores,
  minScores,
  maxScores,
}: ScoreFormProps) => {
  return (
    <>
      {toPairs(scores).map(([ability, value]) => {
        return (
          <div key={ability} className={cn('flex flex-col items-center m-4')}>
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
              range={[minScores[ability], maxScores[ability]]}
            />
          </div>
        );
      })}
    </>
  );
};

const fixCase = (s: string) => `${s[0].toUpperCase()}${s.slice(1)}`;
