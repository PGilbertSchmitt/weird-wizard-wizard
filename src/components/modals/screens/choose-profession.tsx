import { useProfessionIndex } from '@/api/professions';
import { ModalChoiceProps } from '../switch';
import { cn, nth } from '@/lib/utils';
import { Profession } from '@/types/other_info';
import { useState } from 'react';
import { Spinner } from '@/components/ui/neo/spinner';
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from '@/components/ui/neo/tabs';
import { Button } from '@/components/ui/button';
import { ConfirmSelection } from './confirm-selection';

export const ChooseProfession = ({
  character,
  keys,
  modifier,
}: ModalChoiceProps) => {
  const { data: professionCategories } = useProfessionIndex();

  const [selected, setSelected] = useState<Profession[]>([]);

  if (!professionCategories) {
    return <Spinner />;
  }

  const disallowedProfessions = [
    ...character.professions.map((p) => p.id),
    ...selected.map((p) => p.id),
  ];
  const pickProfession = selected.length < keys.length;
  const idxStr = keys.length === 1 ? '' : nth(selected.length);

  return pickProfession ? (
    <div className={cn('w-fit')}>
      <h1>Pick {idxStr} profession</h1>

      <div className={cn('w-full max-w-200')}>
        <Tabs defaultValue={professionCategories[0].name}>
          <TabsList className={cn('w-full')}>
            {professionCategories.map((category) => (
              <TabsTrigger key={category.name} value={category.name}>
                {category.name}
              </TabsTrigger>
            ))}
          </TabsList>
          <div>
            {professionCategories.map((category) => (
              <TabsContent value={category.name}>
                {category.professions.map((profession) => (
                  <Button
                    className={cn('w-full my-2')}
                    disabled={disallowedProfessions.includes(profession.id)}
                    onClick={() => setSelected([...selected, profession])}
                  >
                    <h3>{profession.name}</h3>
                    <p>{profession.description}</p>
                  </Button>
                ))}
              </TabsContent>
            ))}js
          </div>
        </Tabs>
      </div>
    </div>
  ) : (
    <ConfirmSelection
      characterId={character.id}
      modifier={modifier}
      values={selected.map((prof) => prof.id.toString())}
    >
      <ul>
        {selected.map((prof) => (
          <li key={prof.id}>- {prof.name}</li>
        ))}
      </ul>
    </ConfirmSelection>
  );
};
