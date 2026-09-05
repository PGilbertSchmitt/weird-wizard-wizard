import { FullTradition, TraditionIndexItem } from "@/types/magic";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export const useTraditions = () => useQuery({
  queryKey: ['traditions'],
  queryFn: () => invoke<Array<TraditionIndexItem>>('get_tradition_index'),
});

export const useFullTradition = (id: number) => useQuery({
  queryKey: ['tradition', id],
  queryFn: () => invoke<FullTradition>('get_tradition', { id }),
});
