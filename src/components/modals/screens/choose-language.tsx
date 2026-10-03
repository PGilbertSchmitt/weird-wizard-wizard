import { useNonSecretLanguages } from '@/api/languages';
import { ModalChoiceProps } from '../switch';
import { cn, nth } from '@/lib/utils';
import { useMemo, useState } from 'react';
import { Language } from '@/types/other_info';
import { Button } from '@/components/ui/button';
import { ConfirmSelection } from './confirm-selection';

export const ChooseLanguage = ({
  character,
  keys,
  modifier,
}: ModalChoiceProps) => {
  const { data: languages } = useNonSecretLanguages();

  const [selected, setSelected] = useState<Language[]>([]);

  const availableLanguages = useMemo(() => {
    if (!languages) {
      return null;
    }

    const ownedLanguages = character.languages.map((l) => l[0].id);

    return languages.filter((l) => !ownedLanguages.includes(l.id));
  }, [character.languages, languages, selected]);

  if (availableLanguages === null) {
    return null;
  }

  const selectedLanguages = selected.map((l) => l.id);
  const pickLanguage = selected.length < keys.length;
  const idxStr = keys.length === 1 ? '' : nth(selected.length);

  return pickLanguage ? (
    <div className={cn('w-fit')}>
      <h1>Pick {idxStr} language</h1>

      <div className={cn('w-full max-w-200')}>
        {availableLanguages.map((lang) => (
          <Button
            className={cn('m-2 w-full')}
            key={lang.id}
            disabled={selectedLanguages.includes(lang.id)}
            onClick={() => setSelected([...selected, lang])}
          >
            <h2>{lang.name}</h2>
            <p>{lang.description}</p>
          </Button>
        ))}
      </div>
    </div>
  ) : (
    <ConfirmSelection
      characterId={character.id}
      modifier={modifier}
      values={selected.map((lang) => lang.id.toString())}
    >
      <ul>
        {selected.map((lang) => (
          <li key={lang.id}>- {lang.name}</li>
        ))}
      </ul>
    </ConfirmSelection>
  );
};
