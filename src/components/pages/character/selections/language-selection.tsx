import { useMemo } from 'react';
import { SelectionItemProps } from '../character-selection';
import { useLanguagesByIds } from '@/api/languages';
import { Spinner } from '@/components/ui/neo/spinner';

export const LanguageSelection = ({ choices }: SelectionItemProps) => {
  const languageIds = useMemo(() => {
    return choices.map((ch) => parseInt(ch.selection));
  }, [choices]);

  const { data: languages } = useLanguagesByIds(languageIds);
  if (!languages) {
    return <Spinner />;
  }

  return (
    <ul>
      {languages.map((lang) => (
        <li>
          + <b>{lang.name}</b>
        </li>
      ))}
    </ul>
  );
};
