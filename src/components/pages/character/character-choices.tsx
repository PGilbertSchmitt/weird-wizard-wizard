import { FullChoiceModifier, ModalTypes } from '@/components/modals/type';
import { Button } from '@/components/ui/button';
import { useModal } from '@/hooks/modal';
import { cn } from '@/lib/utils';
import { FullCharacter } from '@/types/character';
import {
  ChooseTarget,
  ModifierPathNode,
} from '@/types/modifiers';
import { init, last } from 'ramda';
import { useMemo } from 'react';

interface CharacterChoicesProps {
  character: FullCharacter;
}

export const CharacterChoices = ({
  character,
}: CharacterChoicesProps) => {
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
          <p><i>From reaching level 3</i></p>
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
          <p><i>From reaching level 7</i></p>
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

const choiceString = (target: ChooseTarget): string => {
  switch (target.type) {
    case 'Language': {
      return target.data === 1
        ? `Choose 1 language`
        : `Choose ${target.data} languages.`;
    }
    case 'Profession': {
      return target.data === 1
        ? `Choose 1 profession`
        : `Choose ${target.data} professions.`;
    }
    case 'Tradition': {
      return target.data === 1
        ? `Choose 1 tradition`
        : `Choose ${target.data} traditions.`;
    }
    case 'MagicTalent': {
      const [count, tradition] = target.data;
      return count === 1
        ? `Choose 1 talent from ${tradition} tradition`
        : `Choose ${count} talents from ${tradition} tradition`;
    }
    case 'NoviceSpell': {
      return target.data === 1
        ? `Choose 1 Novice spell`
        : `Choose ${target.data} Novice spells.`;
    }
    case 'ExpertSpell': {
      return target.data === 1
        ? `Choose 1 Expert spell`
        : `Choose ${target.data} Expert spells.`;
    }
    case 'MasterSpell': {
      return target.data === 1
        ? `Choose 1 Master spell`
        : `Choose ${target.data} Master spells.`;
    }
    case 'NoviceSpellFrom': {
      const [count, sources] = target.data;
      const spellSegment =
        count === 1 ? '1 Novice spell' : `${count} Novice spells`;
      const sourceSegment =
        sources.length === 1
          ? `Tradition ${sources[0]}`
          : `Traditions ${orSeparatedStr(sources)}`;
      return `Choose ${spellSegment} from ${sourceSegment}`;
    }
    case 'ExpertSpellFrom': {
      const [count, sources] = target.data;
      const spellSegment =
        count === 1 ? '1 Expert spell' : `${count} Expert spells`;
      const sourceSegment =
        sources.length === 1
          ? `Tradition ${sources[0]}`
          : `Traditions ${orSeparatedStr(sources)}`;
      return `Choose ${spellSegment} from ${sourceSegment}`;
    }
    case 'MasterSpellFrom': {
      const [count, sources] = target.data;
      const spellSegment =
        count === 1 ? '1 Master spell' : `${count} Master spells`;
      const sourceSegment =
        sources.length === 1
          ? `Tradition ${sources[0]}`
          : `Traditions ${orSeparatedStr(sources)}`;
      return `Choose ${spellSegment} from ${sourceSegment}`;
    }
    case 'Score': {
      return target.data === 1
        ? `Choose 1 score improvement`
        : `Choose ${target.data} score improvements`;
    }
    case 'Select': {
      const [count, table] = target.data;
      return count === 1
        ? `Choose 1 effect from ${table}`
        : `Choose ${count} effects from ${table}`;
    }
    case 'Slots': {
      const slotType = target.data.type;
      const slotSymbol = slotType === 'Plus' ? '+' : '*';
      const slotDelta = target.data.data;
      return `Increase spell slot count by ${slotSymbol}${slotDelta}`;
    }
    default: {
      return 'Something else...';
    }
  }
};

const orSeparatedStr = (sources: string[]): string => {
  switch (sources.length) {
    case 0:
      return '';
    case 1:
      return sources[0];
    case 2:
      return `${sources[0]} or ${sources[1]}`;
    default:
      return `${init(sources).join(', ')}, or ${last(sources)}`;
  }
};

const sourceString = ({ type, data }: ModifierPathNode): string => {
  switch (type) {
    case 'LevelLanguage':
      return `${data.path_name} level ${data.level}`;
    case 'LevelTradition':
      return `${data.path_name} level ${data.level}`;
    case 'LevelMagicTalent':
      return `${data.path_name} level ${data.level}`;
    case 'LevelNoviceSpell':
      return `${data.path_name} level ${data.level}`;
    case 'LevelExpertSpell':
      return `${data.path_name} level ${data.level}`;
    case 'LevelMasterSpell':
      return `${data.path_name} level ${data.level}`;
    case 'LevelScore':
      return `reaching level ${data.level}`;
    case 'PathTalent':
      return `Talent '${data.name}' from the ${data.source} path`;
    case 'MagicTalent':
      return `Talent '${data.name}' from the ${data.tradition} tradition`;
    case 'ChoiceSelection':
      return `Decision from ${data.name} option of ${data.label}`;
    // Is this even a thing? I don't think spells can trigger permanent choices. Well, just in case...
    case 'Spell':
      return `${data.name} spell in the ${data.tradition} tradition`;
  }
};
