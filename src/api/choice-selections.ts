import { ChoiceTable } from "@/types/choice_selections";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export const useChoiceTable = (name: string) =>
  useQuery({
    queryKey: ['choice-table', name],
    queryFn: () => invoke<ChoiceTable>('get_choice_table', { name }),
  })