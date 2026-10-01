import { FullCharacter } from '@/types/character';
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from '@/components/ui/neo/tabs';
import { cn } from '@/lib/utils';
import { useMemo } from 'react';
import { partition } from 'ramda';

interface CharacterSelectionsProps {
  character: FullCharacter;
}

export const CharacterSelections = ({
  character,
}: CharacterSelectionsProps) => {
  const [dismissable, other] = useMemo(
    () =>
      partition(
        ([_, choices]) => choices[0].dismissable,
        character.selected_choices,
      ),
    [character],
  );

  return (
    <Tabs
      defaultValue="Dismissable"
      className={cn('flex flex-col items-center')}
    >
      <TabsList>
        <TabsTrigger value="Dismissable">Dismissable</TabsTrigger>
        <TabsTrigger value="Other">Other</TabsTrigger>
      </TabsList>
      <div className={cn('gap-6 w-250 p-4 flex flex-col items-center')}>
        <TabsContent value="Dismissable">
          {dismissable.map(item => (
            <h2>{item[1][0].selection}</h2>
          ))}
        </TabsContent>
        <TabsContent value="Other">
          {other.map(item => (
            <h2>{item[1][0].selection}</h2>
          ))}
        </TabsContent>
      </div>
    </Tabs>
  );
};
