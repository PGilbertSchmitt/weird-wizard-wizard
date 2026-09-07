import { FullSpeedTrait } from '@/types/other_info';
import { FullLevel } from '@/types/path';
import { useMemo } from 'react';
import { TalentCard } from './talent-card';

interface LevelSectionProps {
  level: FullLevel;
}

export const LevelSection = ({ level }: LevelSectionProps) => {
  const attributes = useMemo(() => calculateAttributes(level), [level]);

  return (
    <div className="p-4">
      <h3>Level {level.level}</h3>
      <p>
        {attributes.map((attr) => (
          <span className="pr-4">
            <b>{attr.label}:</b> {attr.value}
          </span>
        ))}
      </p>

      {level.path_talents.map((talent) => (
        <TalentCard talent={talent} />
      ))}
    </div>
  );
};

interface Attribute {
  label: string;
  value: string;
}

const calculateAttributes = (level: FullLevel): Attribute[] => {
  const attrs: Attribute[] = [];

  if (level.add_health > 0) {
    attrs.push({
      label: 'Health',
      value: `+${level.add_health}`,
    });
  }

  if (level.add_nat_def > 0) {
    attrs.push({
      label: 'Natural Defense',
      value: `+${level.add_nat_def}`,
    });
  }

  if (level.add_arm_def > 0) {
    attrs.push({
      label: 'Armored Defense',
      value: `+${level.add_arm_def}`,
    });
  }

  if (level.add_bonus_dmg > 0) {
    attrs.push({
      label: 'Bonus Damage',
      value: `+${level.add_bonus_dmg}d6`,
    });
  }

  const speedString = speedAttrString(level.add_speed, level.speed_traits);
  if (speedString) {
    attrs.push({
      label: 'Speed',
      value: speedString,
    });
  }

  if (level.size) {
    attrs.push({
      label: 'Size',
      value: level.size,
    });
  }

  const traditionString = mixedAttrString(
    level.trad_choices,
    level.traditions.map((t) => t.name),
  );
  if (traditionString) {
    attrs.push({
      label: 'Traditions',
      value: traditionString,
    });
  }

  const languageString = mixedAttrString(level.lang_choices, level.languages);
  if (languageString) {
    attrs.push({
      label: 'Languages',
      value: languageString,
    });
  }

  const spellSubstrings: string[] = [];
  if (level.novice_spells) {
    spellSubstrings.push(`+${level.novice_spells} Novice`);
  }
  if (level.expert_spells) {
    spellSubstrings.push(`+${level.expert_spells} Expert`);
  }
  if (level.master_spells) {
    spellSubstrings.push(`+${level.master_spells} Master`);
  }
  if (spellSubstrings.length > 0) {
    attrs.push({
      label: 'Spell',
      value: spellSubstrings.join(', '),
    });
  }

  return attrs;
};

const speedTraitString = (traits: FullSpeedTrait[]) => {
  if (traits.length === 0) {
    return null;
  }

  const traitSubstrings = traits.map((trait) => {
    const amountString = trait.amount ? `${trait.amount} ${trait.unit}` : null;

    return `${trait.name} ${amountString}`.trim();
  });

  return traitSubstrings.join(', ');
};

const speedAttrString = (add: number, traits: FullSpeedTrait[]) => {
  const traitString = speedTraitString(traits);
  const speedIncreased = add > 0;

  if (!speedIncreased && !traitString) {
    return null;
  }

  if (speedIncreased && !traitString) {
    return `+${add}`;
  }

  if (!speedIncreased && traitString) {
    return traitString;
  }

  return `+${add} (${traitString})`;
};

// Works for both languages and traditions
const mixedAttrString = (choices: number, named: string[]) => {
  const hasChoices = choices > 0;
  const hasNamed = named.length > 0;

  if (!hasChoices && !hasNamed) {
    return null;
  }

  if (hasChoices && !hasNamed) {
    return `Pick ${choices}`;
  }

  if (!hasChoices && hasNamed) {
    return named.join(', ');
  }

  return `${named.join(', ')} plus any ${choices}`;
};
