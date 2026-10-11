import { FullChoiceModifier } from '@/components/modals/type';
import { choiceString, sourceString } from '@/lib/modifier-utils';
import { useMemo } from 'react';

export const useChoiceSource = (modifier: FullChoiceModifier) =>
  useMemo(() => {
    return {
      header: choiceString(modifier.mod_details.target.data),
      source: sourceString(modifier.path_node),
    };
  }, [modifier]);
