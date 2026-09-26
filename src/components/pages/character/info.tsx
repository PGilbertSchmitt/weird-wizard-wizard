import { Button } from '@/components/ui/button';
import { StaticCard } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { FullCharacter } from '@/types/character';
import { HealthAndDamage } from './health-info';
import { AttributeRows, AttributeTable } from '@/components/ui/attribute-table';
import { Fragment, useMemo } from 'react';
import { TooltipText } from '@/components/ui/tooltip-text';
import { init, last, toPairs, values } from 'ramda';
import { Language } from '@/types/other_info';
import { useUpdateCharacterLevel } from '@/api/characters';

interface CharacterInfoProps {
  character: FullCharacter;
}

export const CharacterInfo = ({ character }: CharacterInfoProps) => {
  const { mutateAsync: updateLevel } = useUpdateCharacterLevel(character.id);

  const attributes: AttributeRows = useMemo(
    () => [
      {
        label: 'Defense',
        // Will need to handle armored defense when equipment is implemented
        value: character.nat_def,
      },
      {
        label: 'Speed',
        value: `${character.speed}`,
      },
      {
        label: 'Bonus Damage',
        value: character.bonus_dmg,
      },
      {
        label: 'Strength',
        value: character.strength,
      },
      {
        label: 'Agility',
        value: character.agility,
      },
      {
        label: 'Intellect',
        value: character.intellect,
      },
      {
        label: 'Will',
        value: character.will,
      },
    ],
    [character],
  );

  const traits = useMemo(() => {
    const allTraits: AttributeRows = [];

    const speedTraits = character.speed_traits.map(([trait, source]) => (
      <TooltipText
        key={trait.id}
        label={
          <TooltipWindow>
            <>
              <p className="mb-5">{trait.description}</p>
              <p>
                <i>From {source}</i>
              </p>
            </>
          </TooltipWindow>
        }
      >
        {`${trait.name} ${trait.amount || ''} ${trait.unit || ''}`.trim()}
      </TooltipText>
    ));

    if (speedTraits.length > 0) {
      allTraits.push({
        label: 'Speed Traits',
        value: <CommaSeparated elements={speedTraits} />,
      });
    }

    const senses = character.senses.map(([sense, source]) => (
      <TooltipText
        key={sense.id}
        label={
          <TooltipWindow>
            <>
              <div className="w-100">
                <p className="mb-5">{sense.description}</p>
                <p>
                  <i>From {source}</i>
                </p>
              </div>
            </>
          </TooltipWindow>
        }
      >
        {`${sense.name} ${sense.amount || ''} ${sense.unit || ''}`.trim()}
      </TooltipText>
    ));

    if (senses.length > 0) {
      allTraits.push({
        label: 'Senses',
        value: <CommaSeparated elements={senses} />,
      });
    }

    const collapsedLanguages = character.languages.reduce(
      (
        acc: Record<number, { language: Language; sources: string[] }>,
        [language, source],
      ) => {
        acc[language.id] ||= {
          language,
          sources: [],
        };
        acc[language.id].sources.push(source);
        return acc;
      },
      {},
    );

    const languages = values(collapsedLanguages).map(
      ({ language, sources }) => (
        <TooltipText
          key={language.id}
          label={
            <TooltipWindow>
              <>
                <p className="mb-5">{language.description}</p>
                <p>
                  <i>From {human_comma_string(sources)}</i>
                </p>
              </>
            </TooltipWindow>
          }
        >
          {language.name}
        </TooltipText>
      ),
    );

    if (languages.length > 0) {
      allTraits.push({
        label: 'Languages',
        value: <CommaSeparated elements={languages} />,
      });
    }

    const collapsedImmunities = character.immunities.reduce(
      (acc: Record<string, string[]>, [immunity, source]) => {
        acc[immunity] ||= [];
        acc[immunity].push(source);
        return acc;
      },
      {},
    );

    const immunities = toPairs(collapsedImmunities).map(
      ([immunity, sources]) => (
        <TooltipText
          key={immunity}
          label={
            <TooltipWindow>
              <p>
                <i>From {human_comma_string(sources)}</i>
              </p>
            </TooltipWindow>
          }
        >
          {immunity}
        </TooltipText>
      ),
    );

    if (immunities.length > 0) {
      allTraits.push({
        label: 'Immunities',
        value: <CommaSeparated elements={immunities} />,
      });
    }

    return allTraits;
  }, [character]);

  return (
    <div className={cn('flex flex-col items-center')}>
      <StaticCard className={cn('p-0 w-full')}>
        <div className={cn('flex flex-row justify-center p-2')}>
          <h2>Basic Info</h2>
        </div>

        <Separator />

        <div className={cn('bg-secondary-background text-foreground p-4')}>
          <div className={cn('flex flex-row gap-4')}>
            <div>
              <AttributeTable rows={attributes} />
            </div>

            <div>
              <AttributeTable rows={traits} />
            </div>

            <HealthAndDamage
              characterId={character.id}
              maxHealth={character.max_health}
              curHealth={character.health}
              curDamage={character.damage}
            />
          </div>
        </div>

        <Separator />

        <div className={cn('bg-secondary-background text-foreground p-4')}>
          <h2>Profession: {character.profession.name}</h2>
          <span>({character.profession.category})</span>
          <p>{character.profession.description}</p>
        </div>

        {character.expert_path && (
          <>
            <Separator />

            <div className={cn('bg-secondary-background text-foreground p-4')}>
              <h2>Novice Path: {character.expert_path.name}</h2>
              <span>({character.expert_path.category})</span>
              <p>{character.expert_path.description}</p>
            </div>
          </>
        )}

        {character.master_path && (
          <>
            <Separator />

            <div className={cn('bg-secondary-background text-foreground p-4')}>
              <h2>Profession: {character.master_path.name}</h2>
              <span>({character.master_path.category})</span>
              <p>{character.master_path.description}</p>
            </div>
          </>
        )}
      </StaticCard>

      <div>
        {character.level > 1 && (
          <Button
            className="m-2"
            onClick={() => updateLevel(character.level - 1)}
          >
            Level Down
          </Button>
        )}
        {character.level < 10 && (
          <Button
            className="m-2"
            onClick={() => updateLevel(character.level + 1)}
          >
            Level Up
          </Button>
        )}
      </div>
    </div>
  );
};

interface CommaSeparatedProps {
  elements: Array<React.ReactNode>;
}

const CommaSeparated = ({ elements }: CommaSeparatedProps) => {
  const [first, ...rest] = elements;
  return (
    <span>
      {first}
      {rest.map((other, i) => (
        <Fragment key={i}>, {other}</Fragment>
      ))}
    </span>
  );
};

const human_comma_string = (parts: string[]) => {
  switch (parts.length) {
    case 0:
      return '';
    case 1:
      return parts[0];
    case 2:
      return `${parts[0]} and ${parts[1]}`;
    default: {
      return `${init(parts).join(', ')}, and ${last(parts)}`;
    }
  }
};

const TooltipWindow = ({ children }: { children: React.ReactElement }) => (
  <div className="max-w-150">{children}</div>
);
