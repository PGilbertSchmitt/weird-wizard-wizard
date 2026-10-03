import { FullChoiceModifier } from '@/components/modals/type';
import { CharacterChoice, FullCharacter } from '@/types/character';
import { ScoreSelection } from './selections/score-selection';
import { cn } from '@/lib/utils';
import { StaticCard } from '@/components/ui/card';
import { LanguageSelection } from './selections/language-selection';
import { TraditionSelection } from './selections/tradition-selection';
import { useChoiceSource } from '@/hooks/choice-source';
import { SpellSelection } from './selections/spell-selection';
import { ChoiceTableSelection } from './selections/choice-table-selection';
import { useDeleteChoice } from '@/api/characters';
import { EllipsisVertical } from 'lucide-react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/neo/dropdown-menu';

interface CharacterSelectionProps {
  character: FullCharacter;
  modifier: FullChoiceModifier;
  choices: CharacterChoice[];
  canDismiss?: boolean;
}

export const CharacterSelection = ({
  character,
  modifier,
  choices,
  canDismiss = false,
}: CharacterSelectionProps) => {
  const { mutateAsync: deleteChoice } = useDeleteChoice(character.id);
  return (
    <StaticCard className={cn('w-full relative flex justify-between gap-2')}>
      <SelectionHeader modifier={modifier}>
        <CharacterSelectionSwitch modifier={modifier} choices={choices} />
      </SelectionHeader>

      {canDismiss && (
        <div className={cn('w-fit')}>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <div
                className={cn(
                  'bg-main rounded-full transition-color cursor-pointer hover:brightness-90',
                )}
              >
                <EllipsisVertical strokeWidth="1px" />
              </div>
            </DropdownMenuTrigger>
            <DropdownMenuContent className={cn('m-4')}>
              <DropdownMenuItem onSelect={() => deleteChoice(modifier)}>
                Dismiss
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      )}
    </StaticCard>
  );
};

interface CharacterSelectSwitchProps {
  modifier: FullChoiceModifier;
  choices: CharacterChoice[];
}

const CharacterSelectionSwitch = ({
  modifier,
  choices,
}: CharacterSelectSwitchProps) => {
  const choiceTarget = modifier.mod_details.target.data[0];

  switch (choiceTarget.type) {
    case 'Score': {
      return <ScoreSelection choices={choices} />;
    }
    case 'Language': {
      return <LanguageSelection choices={choices} />;
    }
    case 'Tradition': {
      return <TraditionSelection choices={choices} />;
    }
    case 'NoviceSpell':
    case 'ExpertSpell':
    case 'MasterSpell':
    case 'NoviceSpellFrom':
    case 'ExpertSpellFrom':
    case 'MasterSpellFrom': {
      return <SpellSelection choices={choices} />;
    }
    case 'Select': {
      return <ChoiceTableSelection choices={choices} />;
    }
  }
};

export interface SelectionItemProps {
  choices: CharacterChoice[];
}

interface SelectionHeaderProps {
  modifier: FullChoiceModifier;
  children: React.ReactNode;
}

const SelectionHeader = ({ modifier, children }: SelectionHeaderProps) => {
  const { header, source } = useChoiceSource(modifier);
  return (
    <div className={cn('w-full')}>
      <h2>{header}</h2>
      <p>
        <i>From {source}</i>
      </p>
      <p>Current selections:</p>
      <div className={cn('ml-4')}>{children}</div>
    </div>
  );
};
