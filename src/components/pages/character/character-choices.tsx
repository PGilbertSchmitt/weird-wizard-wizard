import { FullChoiceModifier, ModalTypes } from '@/components/modals/type';
import { Button } from '@/components/ui/button';
import { useModal } from '@/hooks/modal';
import { choiceString, sourceString } from '@/lib/modifier-utils';
import { cn } from '@/lib/utils';
import { FullCharacter } from '@/types/character';
import { useMemo } from 'react';

interface CharacterChoicesProps {
  character: FullCharacter;
}

export const CharacterChoices = ({ character }: CharacterChoicesProps) => {
  const { pushModal } = useModal();

  const choiceItems = useMemo(() => {
    return character.required_choices.flatMap((choice) => {
      const target = choice.mod_details.target;
      if (target.type !== 'Choose') {
        return [];
      }
      const [chooseTarget, _] = target.data;

      const choiceLabel = choiceString(chooseTarget);
      const sourceLabel = sourceString(choice.path_node);

      const choiceMod: FullChoiceModifier = {
        path_node: choice.path_node,
        mod_details: {
          target,
          when: choice.mod_details.when,
          condition: choice.mod_details.condition,
        },
      };

      return {
        modifier: choiceMod,
        choiceLabel,
        sourceLabel,
      };
    });
  }, [character]);

  const pickExpertPath = !character.expert_path && character.level >= 3;
  const pickMasterPath = !character.master_path && character.level >= 7;

  if (choiceItems.length === 0 && !pickExpertPath && !pickMasterPath) {
    return <p>All decisions have been made.</p>;
  }

  return (
    <div className={cn('flex flex-row flex-wrap gap-4 justify-center')}>
      {pickExpertPath && (
        <Button
          key="pick expert path"
          onClick={() =>
            pushModal({
              type: ModalTypes.CHOOSE_PATH,
              character,
              kind: 'Expert',
            })
          }
        >
          <p>Pick Expert Path</p>
          <p>
            <i>From reaching level 3</i>
          </p>
        </Button>
      )}
      {pickMasterPath && (
        <Button
          key="pick master path"
          onClick={() =>
            pushModal({
              type: ModalTypes.CHOOSE_PATH,
              character,
              kind: 'Master',
            })
          }
        >
          <p>Pick Master Path</p>
          <p>
            <i>From reaching level 7</i>
          </p>
        </Button>
      )}
      {choiceItems.map((item, idx) => (
        <Button
          key={idx}
          onClick={() =>
            pushModal({
              type: ModalTypes.CHOOSE,
              character,
              source: item.sourceLabel,
              modifier: item.modifier,
            })
          }
        >
          <p>{item.choiceLabel}</p>
          <p>
            <i>From {item.sourceLabel}</i>
          </p>
        </Button>
      ))}
    </div>
  );
};
