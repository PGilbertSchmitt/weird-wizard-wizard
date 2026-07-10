import { useQuery } from '@tanstack/react-query';
import { Ancestry } from '@/types/path';
import { invoke } from '@tauri-apps/api/core';

export const useAncestry = (id: number) =>
  useQuery({
    queryKey: ['ancestry', id],
    queryFn: async () => invoke<Ancestry>('get_ancestry', { id }),
  });
