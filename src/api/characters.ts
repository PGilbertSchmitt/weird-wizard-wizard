import { CharacterIndexItem } from '@/types/character';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

const CHARACTER_KEYS = {
  characterIndex: ['characterIndex'],
};

export const useCharacterIndex = () =>
  useQuery({
    queryKey: CHARACTER_KEYS.characterIndex,
    queryFn: () => invoke<Array<CharacterIndexItem>>('get_character_index'),
  });
