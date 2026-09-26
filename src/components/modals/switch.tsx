import { cn } from '@/lib/utils';
import { FullChoiceModifier, ModalData, ModalTypes } from './type';
import { ChooseScores } from './screens/choose-scores';
import { StaticCard } from '../ui/card';
import { WithCharacter } from '../providers/character-provider';
import { ChooseTraditionFlow } from './screens/tradition-flow/choose-tradition-flow';
import { FullCharacter } from '@/types/character';
import { ChooseSpellFlow } from './screens/spell-flow/choose-spell-flow';

interface ModalSwitchProps {
  modalData: ModalData;
}

export interface ModalChoiceProps {
  character: FullCharacter;
  modifier: FullChoiceModifier;
  keys: string[];
}

export const ModalSwitch = ({ modalData }: ModalSwitchProps) => {
  if (modalData.type === ModalTypes.ERROR) {
    return (
      <div className={cn('border-border bg-main text-main-foreground p-5')}>
        <p>{modalData.error}</p>
      </div>
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
              return (
                <ChooseSpellFlow
                  {...choiceProps}
                  maxKind='Novice'
                />
              );
            }
            case 'ExpertSpell': {
              return (
                <ChooseSpellFlow
                  {...choiceProps}
                  maxKind='Expert'
                />
              );
            }
            case 'MasterSpell': {
              return (
                <ChooseSpellFlow
                  {...choiceProps}
                  maxKind='Master'
                />
              );
            }
            case 'Tradition': {
              return <ChooseTraditionFlow {...choiceProps} />;
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
