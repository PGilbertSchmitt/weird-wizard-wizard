import { Language } from '@/types/other_info';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

export const useNonSecretLanguages = () =>
  useQuery({
    queryKey: ['non-secret-languages'],
    queryFn: () => invoke<Array<Language>>('get_non_secret_languages'),
  });

export const useLanguagesByIds = (ids: number[]) =>
  useQuery({
    queryKey: ['languages', ids.sort((a, b) => a - b)],
    queryFn: () => invoke<Array<Language>>('get_languages_by_ids', { ids }),
  });
