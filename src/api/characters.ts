import {
  CharacterIndexItem,
  CreateCharacter,
  FullCharacter,
} from '@/types/character';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

const CHARACTER_KEYS = {
  characterIndex: ['characterIndex'],
  character: (id: number) => ['character', id],
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

export const useCharacter = (id: number) =>
  useQuery({
    queryKey: CHARACTER_KEYS.character(id),
    queryFn: () => invoke<FullCharacter>('get_full_character', { id }),
    enabled: id >= 0,
  });
