import { useModal } from '@/hooks/modal';
import {
  CharacterIndexItem,
  CreateCharacter,
  FullCharacter,
} from '@/types/character';
import { FullModifier } from '@/types/modifiers';
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

export const useCharacter = (id: number) => {
  const { pushError } = useModal();
  return useQuery({
    queryKey: CHARACTER_KEYS.character(id),
    queryFn: () => invoke<FullCharacter>('get_full_character', { id }).catch(err => pushError(err.toString())),
    enabled: id >= 0,
    retry: false,
  });
}

interface UpdateCharacterHealthParams {
  health: number;
  damage: number;
}

export const useUpdateCharacterHealth = (id: number) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ health, damage }: UpdateCharacterHealthParams) =>
      invoke<void>('update_character_health', { id, health, damage }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: CHARACTER_KEYS.character(id),
      });
    },
  });
};

export const useUpdateCharacterLevel = (id: number) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (level: number) =>
      invoke<void>('update_character_level', { id, level }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: CHARACTER_KEYS.character(id),
      });
    },
  });
};

interface SaveChoiceParams {
  modifier: FullModifier;
  values: Array<string>;
}

export const useSaveChoice = (characterId: number) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ modifier, values }: SaveChoiceParams) =>
      invoke('save_choice', {
        characterId: characterId,
        modifier,
        values,
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: CHARACTER_KEYS.character(characterId),
      });
    },
  });
};
