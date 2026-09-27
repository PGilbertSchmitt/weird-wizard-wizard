import { FullCharacter } from '@/types/character';
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from '@/components/ui/neo/tabs';
import { useMemo } from 'react';
import { FullPathTalent } from '@/types/path';
import { FullMagicTalent } from '@/types/magic';
import { toPairs } from 'ramda';
import { cn } from '@/lib/utils';
import { MagicTalentCard } from '../tome/magic-talent-card';
import { PathTalentCard } from '../path-talent-card';

interface CharacterTalentsProps {
  character: FullCharacter;
}

export const CharacterTalents = ({ character }: CharacterTalentsProps) => {
  const talentsBySource = useMemo(() => {
    return toPairs(
      character.path_talents.reduce(
        (acc, [talent]) => {
          acc[talent.source] ||= [];
          acc[talent.source].push(talent);
          return acc;
        },
        {} as Record<string, Array<FullPathTalent>>,
      ),
    );
  }, [character.path_talents]);

  const talentsByTradition = useMemo(() => {
    return toPairs(
      character.magic_talents.reduce(
        (acc, [talent]) => {
          acc[talent.tradition_name] ||= [];
          acc[talent.tradition_name].push(talent);
          return acc;
        },
        {} as Record<string, Array<FullMagicTalent>>,
      ),
    );
  }, [character.magic_talents]);

  if (talentsBySource.length === 0 && talentsByTradition.length === 0) {
    return (
      <p>No talents.</p>
    )
  }

  const defaultValue = talentsBySource.length > 0
    ? sourceKey(talentsBySource[0][0])
    : traditionKey(talentsByTradition[0][0]);

  return (
    <Tabs defaultValue={defaultValue} className={cn('flex flex-col items-center')}>
      <TabsList className={cn('flex flex-wrap h-fit gap-x-5')}>
        {talentsBySource.map(([source, _]) => {
          const key = sourceKey(source);
          return (
            <TabsTrigger key={key} value={key}>
              {source}
            </TabsTrigger>
          );
        })}
        {talentsByTradition.map(([tradition, _]) => {
          const key = traditionKey(tradition);
          return (
            <TabsTrigger key={key} value={key}>
              {tradition} Tradition
            </TabsTrigger>
          );
        })}
      </TabsList>
      {talentsBySource.map(([source, talents]) => {
        const key = sourceKey(source);
        return (
          <TabsContent key={key} value={key} className={cn('flex flex-col gap-4 w-full')}>
            {talents.map((talent) => (
              <PathTalentCard
                key={talent.id}
                talent={talent}
              />
            ))}
          </TabsContent>
        );
      })}
      {talentsByTradition.map(([tradition, talents]) => {
        const key = traditionKey(tradition);
        return (
          <TabsContent key={key} value={key} className={cn('flex flex-col gap-4 w-full')}>
            {talents.map((talent) => (
              <MagicTalentCard
                key={talent.id}
                talent={talent}
              />
            ))}
          </TabsContent>
        );
      })}
    </Tabs>
  );
};

const sourceKey = (source: string) => `${source}-path`;
const traditionKey = (source: string) => `${source}-tradition`;
