import { useModal } from '@/hooks/modal';
import {
  CharacterIndexItem,
  CreateCharacter,
  FullCharacter,
} from '@/types/character';
import { SpellSlotInfo } from '@/types/magic';
import { FullModifier, WhenDuration } from '@/types/modifiers';
import {
  QueryClient,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

const CHARACTER_KEYS = {
  characterIndex: ['characterIndex'],
  character: (id: number) => ['character', id],
};

const invalidateCharacterDependentQueries = (
  queryClient: QueryClient,
  characterId: number,
) => {
  queryClient.invalidateQueries({
    queryKey: CHARACTER_KEYS.character(characterId),
  });
  invalidateSpellSlotQueries(queryClient, characterId);
};

const invalidateSpellSlotQueries = (
  queryClient: QueryClient,
  characterId: number,
) => {
  queryClient.invalidateQueries({
    queryKey: ['spellSlots', characterId],
  });
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
    queryFn: () =>
      invoke<FullCharacter>('get_full_character', { id }).catch((err) =>
        pushError(err.toString()),
      ),
    enabled: id >= 0,
    retry: false,
  });
};

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
      invalidateCharacterDependentQueries(queryClient, id);
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
      invalidateCharacterDependentQueries(queryClient, characterId);
    },
  });
};

export const useDeleteChoice = (characterId: number) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (modifier: FullModifier) =>
      invoke('delete_choice', {
        characterId: characterId,
        modifier,
      }),
    onSuccess: () => {
      invalidateCharacterDependentQueries(queryClient, characterId);
    },
  });
};

export const useSetPath = (id: number) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (pathId: number) =>
      invoke('update_character_path', { id, pathId }),
    onSuccess: () => {
      invalidateCharacterDependentQueries(queryClient, id);
    },
  });
};

export const useSpellSlots = (characterId: number, spellId: number) =>
  useQuery({
    queryKey: ['spellSlots', characterId, spellId],
    queryFn: () =>
      invoke<SpellSlotInfo | null>('get_spell_slots', { characterId, spellId }),
  });

interface CastSpellParams {
  spellId: number;
  permCount: number;
  tempCount: number;
}

export const useCastSpell = (characterId: number) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (params: CastSpellParams) =>
      invoke('cast_spell', { ...params, characterId }),
    onSuccess: () => {
      invalidateSpellSlotQueries(queryClient, characterId);
    },
  });
};

export const usePassTime = (characterId: number) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (duration: WhenDuration) =>
      invoke('pass_time', { characterId, duration }),
    onSuccess: () => {
      invalidateCharacterDependentQueries(queryClient, characterId);
    },
  });
}
