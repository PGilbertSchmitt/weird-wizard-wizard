import { useCharacter } from '@/api/characters';
import { FullCharacter } from '@/types/character';

interface WithCharacterProps {
  characterId: number;
  onRender: (character: FullCharacter) => React.ReactNode;
}

export const WithCharacter = ({
  characterId,
  onRender,
}: WithCharacterProps) => {
  const { data: character } = useCharacter(characterId);
  if (character) {
    return onRender(character);
  } else {
    return null;
  }
};
