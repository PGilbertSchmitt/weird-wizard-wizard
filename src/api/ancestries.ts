import { useQuery } from '@tanstack/react-query';
import { FullAncestry } from '@/types/path';
import { invoke } from '@tauri-apps/api/core';

export const useAncestry = (id: number) =>
  useQuery({
    queryKey: ['ancestry', id],
    queryFn: async () => invoke<FullAncestry>('get_full_ancestry', { id }),
  });

export const useAllAncestries = () =>
  useQuery({
    queryKey: ['ancestries'],
    queryFn: async () => invoke<FullAncestry[]>('get_all_ancestries'),
  });
