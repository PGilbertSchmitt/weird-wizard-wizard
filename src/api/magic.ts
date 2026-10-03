import {
  FullMagicTalent,
  FullSpell,
  FullTradition,
  TraditionIndexItem,
} from '@/types/magic';
import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

export const useTraditions = () =>
  useQuery({
    queryKey: ['traditions'],
    queryFn: () => invoke<Array<TraditionIndexItem>>('get_tradition_index'),
  });

export const useFullTradition = (id: number) =>
  useQuery({
    queryKey: ['tradition', id],
    queryFn: () => invoke<FullTradition>('get_tradition', { id }),
  });

export const useMagicTalents = (traditionId: number) =>
  useQuery({
    queryKey: ['magic_talents', traditionId],
    queryFn: () =>
      invoke<Array<FullMagicTalent>>('get_magic_talents_for_tradition', {
        traditionId,
      }),
  });

export const useSpellsForTradition = (traditionId: number) =>
  useQuery({
    queryKey: ['spells', traditionId],
    queryFn: () =>
      invoke<Array<FullSpell>>('get_spells_for_tradition', { traditionId }),
  });

export const useTraditionName = (id: number) =>
  useQuery({
    queryKey: ['tradition_name', id],
    queryFn: () =>
      invoke<String>('get_tradition_name', { id }),
  });

export const useSpellName = (id: number) =>
  useQuery({
    queryKey: ['spell_name', id],
    queryFn: () =>
      invoke<String>('get_spell_name', { id }),
  });

export const useMagicTalentName = (id: number) =>
  useQuery({
    queryKey: ['magic_talent_name', id],
    queryFn: () =>
      invoke<String>('get_magic_talent_name', { id }),
  });

