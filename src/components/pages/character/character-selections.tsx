import { isChoiceMod } from '@/lib/modifier-utils';
import { cn } from '@/lib/utils';
import { CharacterChoice, FullCharacter } from '@/types/character';
import { useMemo, useState } from 'react';
import { CharacterSelection } from './character-selection';
import { FullChoiceModifier } from '@/components/modals/type';
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from '@/components/ui/neo/tabs';
import { Switch } from '@/components/ui/neo/switch';
import { Label } from '@/components/ui/neo/label';

interface CharacterSelectionsProps {
  character: FullCharacter;
}

type Selection = [FullChoiceModifier, CharacterChoice[]];

export const CharacterSelections = ({
  character,
}: CharacterSelectionsProps) => {
  // This allows the player to dismiss non-dismissable choices
  const [allowDismiss, setAllowDismiss] = useState(false);

  // Not for controlling the tabs (they're self controlled), but for rendering the dismiss switch
  const [showSwitch, setShowSwitch] = useState(false);

  const [dismissableChoices, nonDismissableChoices] = useMemo(() => {
    const dismissable: Selection[] = [];
    const both: Selection[] = [];
    const timeSensitive: Selection[] = [];
    const permanent: Selection[] = [];

    for (const [modifier, selections] of character.selected_choices) {
      if (isChoiceMod(modifier)) {
        switch (modifier.mod_details.when.type) {
          case 'CastOnce': {
            dismissable.push([modifier, selections]);
            break;
          }
          case 'CastTime': {
            timeSensitive.push([modifier, selections]);
            break;
          }
          case 'CastTimeDismiss': {
            both.push([modifier, selections]);
            break;
          }
          case 'Permanent': {
            permanent.push([modifier, selections]);
            break;
          }
        }
      }
    }

    return [
      // Can be dismissed with a button
      [...dismissable, ...both],
      // Cannot be dismissed manually
      [...timeSensitive, ...permanent],
    ];
  }, [character]);

  return (
    <Tabs
      defaultValue="Dismissable"
      className={cn('flex flex-col items-center')}
      onValueChange={(value) => setShowSwitch(value === 'Non Dismissable')}
    >
      <div className={cn('relative')}>
        <TabsList>
          <TabsTrigger value="Dismissable">Dismissable</TabsTrigger>
          <TabsTrigger value="Non Dismissable">Non Dismissable</TabsTrigger>
        </TabsList>
        {showSwitch && (
          <div
            className={cn(
              'absolute inset-y-0 -right-45 top-0 flex justify-center items-center',
            )}
          >
            <Switch
              id="allow-dismiss"
              checked={allowDismiss}
              onCheckedChange={setAllowDismiss}
            />
            <Label htmlFor="allow-dismiss" className="ml-2">
              Allow dismissing
            </Label>
          </div>
        )}
      </div>
      <TabsContent value="Dismissable">
        <SelectionList>
          {dismissableChoices.map(([modifier, choices]) => (
            <CharacterSelection
              key={choices[0].id}
              character={character}
              modifier={modifier}
              choices={choices}
              canDismiss
            />
          ))}
        </SelectionList>
      </TabsContent>
      <TabsContent value="Non Dismissable">
        <SelectionList>
          {nonDismissableChoices.map(([modifier, choices]) => (
            <CharacterSelection
              key={choices[0].id}
              character={character}
              modifier={modifier}
              choices={choices}
              canDismiss={allowDismiss}
            />
          ))}
        </SelectionList>
      </TabsContent>
    </Tabs>
  );
};

const SelectionList = ({ children }: { children: React.ReactNode }) => (
  <div className={cn('w-150 flex flex-col gap-4')}>{children}</div>
);
