import { Language } from '@/types/other_info';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

export const useNonSecretLanguages = () =>
  useQuery({
    queryKey: ['non-secret-languages'],
    queryFn: () => invoke<Array<Language>>('get_non_secret_languages'),
  });
