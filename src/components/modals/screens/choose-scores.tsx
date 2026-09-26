import { Button } from '@/components/ui/button';
import { ScoreForm, Scores } from '@/components/ui/score-form';
import { cn } from '@/lib/utils';
import { min, toPairs } from 'ramda';
import { useState } from 'react';
import { useModal } from '@/hooks/modal';
import { useSaveChoice } from '@/api/characters';
import { ModalChoiceProps } from '../switch';

export const ChooseScores = ({
  keys,
  character,
  modifier,
}: ModalChoiceProps) => {
  const { mutateAsync: saveChoice } = useSaveChoice(character.id);

  const amount = keys.length;
  const [scores, setScores] = useState<Scores>({
    strength: character.strength,
    agility: character.agility,
    intellect: character.intellect,
    will: character.will,
  });
  const { popModal, pushModal } = useModal();

  const highestSumScore =
    character.strength +
    character.agility +
    character.intellect +
    character.will +
    amount;

  const currentSumScore =
    scores.strength + scores.agility + scores.intellect + scores.will;

  const pointsRemaining = highestSumScore - currentSumScore;

  const header =
    amount === 1
      ? `Pick a score to increase`
      : `Pick ${amount} scores to increase`;

  const minScores: Scores = {
    strength: character.strength,
    agility: character.agility,
    intellect: character.intellect,
    will: character.will,
  };

  // In the rules, there is no point where a character must pick more than 4 scores at once,
  // and the same score can't be picked twice in the same instance. But just in case someone
  // creates a custom rule where more than 4 scores must be picked at the same time, the
  // theoretical increase for any given score must increase to prevent a soft-lock.
  const maxIncrease = Math.ceil(amount / 4);
  const maxScores: Scores = {
    strength: character.strength + min(pointsRemaining, maxIncrease),
    agility: character.agility + min(pointsRemaining, maxIncrease),
    intellect: character.intellect + min(pointsRemaining, maxIncrease),
    will: character.will + min(pointsRemaining, maxIncrease),
  };

  // This is a little convoluted. This form only allows as many increments as there are keys
  // to save to, and it's technically possible to increase the same score multiple times.
  const onSave = () => {
    const incrementedScores: string[] = [];
    const tmpScores = { ...scores };

    for (let _ of keys) {
      for (let [ability, value] of toPairs(tmpScores)) {
        if (value > minScores[ability]) {
          incrementedScores.push(ability);
          tmpScores[ability] -= 1;
          break;
        }
      }
    }

    saveChoice({
      modifier,
      values: incrementedScores,
    })
      .then(popModal)
      .catch((err) => {
        popModal();
        pushModal({
          type: 'Error',
          error: err,
        });
      });
  };

  return (
    <div className={cn('w-fit flex flex-col items-center gap-4')}>
      <h1>
        {header} for {character.name}
      </h1>

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

      <Button
        className={cn('px-10 py-2')}
        disabled={pointsRemaining > 0}
        onClick={onSave}
      >
        Confirm
      </Button>
    </div>
  );
};
