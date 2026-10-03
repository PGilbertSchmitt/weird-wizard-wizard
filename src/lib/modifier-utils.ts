import { FullChoiceModifier } from '@/components/modals/type';
import {
  ChooseTarget,
  FullModifier,
  ModifierPathNode,
} from '@/types/modifiers';
import { orSeparatedStr } from './utils';

export const isChoiceMod = (
  modifier: FullModifier,
): modifier is FullChoiceModifier => {
  return modifier.mod_details.target.type === 'Choose';
};

export const choiceString = (target: ChooseTarget): string => {
  switch (target.type) {
    case 'Language': {
      return target.data === 1
        ? `Choose 1 language`
        : `Choose ${target.data} languages`;
    }
    case 'Profession': {
      return target.data === 1
        ? `Choose 1 profession`
        : `Choose ${target.data} professions`;
    }
    case 'Tradition': {
      return target.data === 1
        ? `Choose 1 tradition`
        : `Choose ${target.data} traditions`;
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
        : `Choose ${target.data} Novice spells`;
    }
    case 'ExpertSpell': {
      return target.data === 1
        ? `Choose 1 Expert spell`
        : `Choose ${target.data} Expert spells`;
    }
    case 'MasterSpell': {
      return target.data === 1
        ? `Choose 1 Master spell`
        : `Choose ${target.data} Master spells`;
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

export const sourceString = ({ type, data }: ModifierPathNode): string => {
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
