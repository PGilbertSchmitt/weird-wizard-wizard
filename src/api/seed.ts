import { useMutation } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

export const useInitSeed = (filepath: string) =>
  useMutation({
    mutationFn: () => invoke('init_seed', { filepath }),
  });
