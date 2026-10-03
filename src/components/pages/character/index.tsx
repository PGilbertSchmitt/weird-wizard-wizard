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
import { CharacterSelections } from './character-selections';

export const CharacterPage = () => {
  const params = useParams();
  const id = parseInt(params['id'] || '-1');
  const { data: character, error } = useCharacter(id);

  if (error) {
    return (
      <div className="flex flex-col items-center gap-5">
        <h1>Error when rendering character with ID {id}</h1>
        <p>
          <i>{error.message}</i>
        </p>
        <h3>
          Contact your local 'Gilly dah Fish' or other reputable debugger.
        </h3>
      </div>
    );
  }

  if (character === undefined) {
    return null;
  }

  const highestPath =
    character.master_path || character.expert_path || character.novice_path;
  const characterLabel =
    character.ancestry.name === highestPath.name
      ? character.ancestry.name
      : `${character.ancestry.name} ${highestPath.name}`;

  const pickExpertPath = !character.expert_path && character.level >= 3;
  const pickMasterPath = !character.master_path && character.level >= 7;
  let choiceCount = character.required_choices.length;
  pickExpertPath && choiceCount++;
  pickMasterPath && choiceCount++;

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
          <TabsTrigger value="Pending Choices">Pending Choices ({choiceCount})</TabsTrigger>
          <TabsTrigger value="Selected Choices">Selected Choices</TabsTrigger>
        </TabsList>
        <div className={cn('gap-6 w-250 p-4 flex flex-col items-center')}>
          <TabsContent value="Info">
            <CharacterInfo character={character} />
          </TabsContent>
          <TabsContent value="Talents">
            <CharacterTalents character={character} />
          </TabsContent>
          <TabsContent value="Spells">
            <CharacterSpells spells={character.spells.map((s) => s[0])} />
          </TabsContent>
          <TabsContent value="Pending Choices">
            <CharacterChoices
              character={character}
            />
          </TabsContent>
          <TabsContent value="Selected Choices">
            <CharacterSelections
              character={character}
            />
          </TabsContent>
        </div>
      </Tabs>
    </div>
  );
};
