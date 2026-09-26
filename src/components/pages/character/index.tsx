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
import { CharacterChoices } from './character-choices';
import { CharacterTalents } from './character-talents';
import { CharacterSpells } from './character-spells';

export const CharacterPage = () => {
  const params = useParams();
  const id = parseInt(params['id'] || '-1');
  const { data: character } = useCharacter(id);

  console.log('Character', character);

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
          <TabsTrigger value="Choices">Choices ({character.required_choices.length})</TabsTrigger>
        </TabsList>
        <div className={cn('gap-6 w-250 p-4 flex flex-col items-center')}>
          <TabsContent value="Info">
            <CharacterInfo character={character} />
          </TabsContent>
          <TabsContent value="Talents">
            <CharacterTalents character={character} />
          </TabsContent>
          <TabsContent value="Spells">
            <CharacterSpells spells={character.spells.map(s => s[0])} />
          </TabsContent>
          <TabsContent value="Choices">
            <CharacterChoices
              characterId={character.id}
              choices={character.required_choices}
            />
          </TabsContent>
        </div>
      </Tabs>
    </div>
  );
};
