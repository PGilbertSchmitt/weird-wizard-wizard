import { Button } from '@/components/ui/button';
import { StaticCard } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { FullCharacter } from '@/types/character';
import { HealthAndDamage } from './health-info';

interface CharacterInfoProps {
  character: FullCharacter;
}

export const CharacterInfo = ({ character }: CharacterInfoProps) => {
  return (
    <div className={cn('flex flex-col items-center')}>
      <StaticCard className={cn('my-4 p-0 w-full')}>
        <div className={cn('flex flex-row justify-center p-2')}>
          <h2>Basic Info</h2>
        </div>

        <Separator />

        <div className={cn('bg-secondary-background text-foreground p-4')}>
          <HealthAndDamage
            characterId={character.id}
            maxHealth={character.max_health}
            curHealth={character.health}
            curDamage={character.damage}
          />
        </div>
      </StaticCard>

      <div>
        <Button className="m-2">Level Up</Button>
        <Button className="m-2">Level Down</Button>
      </div>
    </div>
  );
};
