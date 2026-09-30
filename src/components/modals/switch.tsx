import { cn } from '@/lib/utils';
import { FullChoiceModifier, ModalData, ModalTypes } from './type';
import { ChooseScores } from './screens/choose-scores';
import { StaticCard } from '../ui/card';
import { WithCharacter } from '../providers/character-provider';
import { ChooseTraditionFlow } from './screens/tradition-flow/choose-tradition-flow';
import { FullCharacter } from '@/types/character';
import { ChooseSpellFlow } from './screens/spell-flow/choose-spell-flow';
import { ChooseLanguage } from './screens/choose-language';
import { Button } from '../ui/button';
import { useModal } from '@/hooks/modal';
import { ChooseSelection } from './screens/choose-selection';

interface ModalSwitchProps {
  modalData: ModalData;
}

export interface ModalChoiceProps {
  character: FullCharacter;
  modifier: FullChoiceModifier;
  keys: string[];
}

export const ModalSwitch = ({ modalData }: ModalSwitchProps) => {
  const { popModal } = useModal();

  if (modalData.type === ModalTypes.ERROR) {
    return (
      <StaticCard
        className={cn(
          'bg-secondary-background text-foreground py-10 px-20 w-max-300',
        )}
      >
        <h2>ERROR</h2>
        <p className={cn('my-4')}>{modalData.error}</p>
        <Button onClick={popModal}>Aw, ok...</Button>
      </StaticCard>
    );
  }

  const [chooseTarget, keys] = modalData.modifier.mod_details.target.data;

  return (
    <ModalContainer>
      <WithCharacter
        characterId={modalData.characterId}
        onRender={(character) => {
          const choiceProps: ModalChoiceProps = {
            character: character,
            keys: keys,
            modifier: modalData.modifier,
          };

          switch (chooseTarget.type) {
            case 'Score': {
              return <ChooseScores {...choiceProps} />;
            }
            case 'NoviceSpell': {
              return <ChooseSpellFlow {...choiceProps} maxKind="Novice" />;
            }
            case 'ExpertSpell': {
              return <ChooseSpellFlow {...choiceProps} maxKind="Expert" />;
            }
            case 'MasterSpell': {
              return <ChooseSpellFlow {...choiceProps} maxKind="Master" />;
            }
            case 'Tradition': {
              return <ChooseTraditionFlow {...choiceProps} />;
            }
            case 'Language': {
              return <ChooseLanguage {...choiceProps} />;
            }
            case 'Select': {
              return (
                <ChooseSelection
                  {...choiceProps}
                  selectionName={chooseTarget.data[1]}
                />
              );
            }
            case 'NoviceSpellFrom':
            case 'ExpertSpellFrom':
            case 'MasterSpellFrom': {
              return <h1>TODO</h1>;
            }
          }
        }}
      />
    </ModalContainer>
  );
};

const ModalContainer = ({ children }: { children: React.ReactNode }) => (
  <StaticCard
    className={cn(
      'p-10 w-fit max-w-300 bg-secondary-background text-foreground',
    )}
  >
    {children}
  </StaticCard>
);
