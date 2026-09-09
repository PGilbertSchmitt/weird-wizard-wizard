import { useQuery } from '@tanstack/react-query';
import { FullProfessionCategory } from '@/types/other_info';
import { invoke } from '@tauri-apps/api/core';

export const useProfessionIndex = () =>
  useQuery({
    queryKey: ['professions_index'],
    queryFn: () => invoke<FullProfessionCategory[]>('get_all_professions'),
  });
