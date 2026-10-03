import { FullPath, NovicePath, PathIndexItem } from '@/types/path';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

export const usePathIndex = () =>
  useQuery({
    queryKey: ['path_index'],
    queryFn: () => invoke<PathIndexItem[]>('get_path_index'),
  });

export const usePathsForCategory = (kind: string, category: string) =>
  useQuery({
    queryKey: ['paths_for_category', kind, category],
    queryFn: () =>
      invoke<FullPath[]>('get_paths_for_kind_and_category', {
        kind,
        category,
      }),
  });

export const useCategoriesForKind = (kind: string) =>
  useQuery({
    queryKey: ['path_categories_for_kind', kind],
    queryFn: () => invoke<string[]>('get_path_categories_for_kind', { kind }),
  });

export const useFullNovicePath = (id: number) =>
  useQuery({
    queryKey: ['novice_path', id],
    queryFn: () => invoke<NovicePath>('get_novice_path', { id }),
  });
