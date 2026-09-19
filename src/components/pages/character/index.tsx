import { useCharacter } from '@/api/characters';
import { cn } from '@/lib/utils';
import { useParams } from 'react-router';
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from '@/components/ui/neo/tabs';
import { CharacterInfo } from './info';

export const CharacterPage = () => {
  const params = useParams();
  const id = parseInt(params['id'] || '-1');
  const { data: character } = useCharacter(id);

  console.log(character);

  if (character === undefined) {
    return null;
  }

  const highestPath =
    character.master_path || character.expert_path || character.novice_path;
  const characterLabel =
    character.ancestry.name === highestPath.name
      ? character.ancestry.name
      : `${character.ancestry.name} ${highestPath.name}`;

  return (
    <div>
      <div className={cn('flex flex-col items-center mb-8')}>
        <h1>{character.name}</h1>
        <p>
          Level {character.level} {characterLabel}
        </p>
      </div>

      <Tabs defaultValue="Info" className={cn('flex flex-col items-center')}>
        <TabsList>
          <TabsTrigger value="Info">Info</TabsTrigger>
          <TabsTrigger value="Talents">Talents</TabsTrigger>
          <TabsTrigger value="Spells">Spells</TabsTrigger>
          <TabsTrigger value="Choices">Choices</TabsTrigger>
        </TabsList>
        <div className={cn('gap-6 w-250 px-4')}>
          <TabsContent value="Info">
            <CharacterInfo character={character} />
          </TabsContent>
          <TabsContent value="Talents">
            <p>value="Talents"</p>
          </TabsContent>
          <TabsContent value="Spells">
            <p>value="Spells"</p>
          </TabsContent>
          <TabsContent value="Choices">
            <p>value="Choices"</p>
          </TabsContent>
        </div>
      </Tabs>
    </div>
  );
};
