import { Button } from '@/components/ui/button';
import { StaticCard } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { FullCharacter } from '@/types/character';
import { HealthAndDamage } from './health-info';
import { AttributeRows, AttributeTable } from '@/components/ui/attribute-table';
import { useMemo } from 'react';
import { BadgeWithTooltip } from '@/components/ui/badge';

interface CharacterInfoProps {
  character: FullCharacter;
}

export const CharacterInfo = ({ character }: CharacterInfoProps) => {
  const attributes: AttributeRows = useMemo(
    () => [
      {
        label: 'Defense',
        // Will need to handle armored defense when equipment is implemented
        value: character.nat_def,
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
    const allTraits: AttributeRows = [
      {
        label: 'Speed',
        value: `${character.speed}`,
      },
    ];

    const speedTraits = character.speed_traits.map(([trait, source]) => (
      <BadgeWithTooltip
        key={trait.id}
        label={
          <div>
            <p>{trait.description}</p>
            <p>
              <i>From {source}</i>
            </p>
          </div>
        }
      >
        {`${trait.name} ${trait.amount || ''} ${trait.unit || ''}`.trim()}
      </BadgeWithTooltip>
    ));

    if (speedTraits.length > 0) {
      allTraits.push({
        label: 'Speed Traits',
        value: speedTraits,
      });
    }

    return allTraits;
  }, [character]);

  return (
    <div className={cn('flex flex-col items-center')}>
      <StaticCard className={cn('my-4 p-0 w-full')}>
        <div className={cn('flex flex-row justify-center p-2')}>
          <h2>Basic Info</h2>
        </div>

        <Separator />

        <div className={cn('bg-secondary-background text-foreground p-4')}>
          <div className={cn('flex flex-row')}>
            <HealthAndDamage
              characterId={character.id}
              maxHealth={character.max_health}
              curHealth={character.health}
              curDamage={character.damage}
            />

            <div className={cn('border border-border')}>
              <AttributeTable rows={attributes} />
            </div>

            <div className={cn('border border-border')}>
              <AttributeTable rows={traits} />
            </div>
          </div>
        </div>
      </StaticCard>

      <div>
        <Button className="m-2">Level Up</Button>
        <Button className="m-2">Level Down</Button>
      </div>
    </div>
  );
};
