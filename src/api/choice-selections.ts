import { ChoiceTable, FullChoice } from '@/types/choice_selections';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

export const useChoiceTable = (name: string) =>
  useQuery({
    queryKey: ['choice-table', name],
    queryFn: () => invoke<ChoiceTable>('get_choice_table', { name }),
  });

export const useChoiceSelection = (id: number) =>
  useQuery({
    queryKey: ['choice-selection', id],
    queryFn: () => invoke<FullChoice>('get_choice_selection', { id }),
  });
