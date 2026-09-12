import { CharacterIndexItem, CreateCharacter } from '@/types/character';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

const CHARACTER_KEYS = {
  characterIndex: ['characterIndex'],
};

export const useCharacterIndex = () =>
  useQuery({
    queryKey: CHARACTER_KEYS.characterIndex,
    queryFn: () => invoke<Array<CharacterIndexItem>>('get_character_index'),
  });

export const useCreateCharacter = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (characterInfo: CreateCharacter) =>
      invoke<number>('create_character', { characterInfo }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: CHARACTER_KEYS.characterIndex,
      });
    },
  });
};
