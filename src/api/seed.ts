import { useMutation, useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

export const useInitSeed = () =>
  useMutation({
    mutationFn: (filepath: string) => invoke('init_seed', { filepath }),
  });

export const useRunSeed = () =>
  useMutation({
    mutationFn: () => invoke('run_seed'),
  });

export const IS_SEEDED_KEY = ['isSeeded'];

export const useIsSeeded = () =>
  useQuery({
    queryKey: IS_SEEDED_KEY,
    queryFn: () => invoke('check_seed'),
  });
