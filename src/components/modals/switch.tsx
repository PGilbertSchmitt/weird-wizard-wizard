import { cn } from '@/lib/utils';
import { FullChoiceModifier, ModalData, ModalTypes } from './type';
import { ChooseScores } from './screens/choose-scores';
import { StaticCard } from '../ui/card';
import { ChooseTraditionFlow } from './screens/tradition-flow/choose-tradition-flow';
import { FullCharacter } from '@/types/character';
import { ChooseSpellFlow } from './screens/spell-flow/choose-spell-flow';
import { ChooseLanguage } from './screens/choose-language';
import { Button } from '../ui/button';
import { useModal } from '@/hooks/modal';
import { ChooseSelection } from './screens/choose-selection';
import { X } from 'lucide-react';
import { ChoosePath } from './screens/choose-path';
import { ChooseProfession } from './screens/choose-profession';

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

  if (modalData.type === ModalTypes.CHOOSE_PATH) {
    return (
      <ModalContainer onClose={popModal}>
        <ChoosePath character={modalData.character} kind={modalData.kind} />
      </ModalContainer>
    );
  }

  const [chooseTarget, keys] = modalData.modifier.mod_details.target.data;

  const choiceProps: ModalChoiceProps = {
    character: modalData.character,
    keys: keys,
    modifier: modalData.modifier,
  };

  return (
    <ModalContainer onClose={popModal}>
      {(() => {
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
          case 'MagicTalent': {
            return (
              <ChooseTraditionFlow
                {...choiceProps}
                limitedTraditions={chooseTarget.data[1]}
              />
            );
          }
          case 'Language': {
            return <ChooseLanguage {...choiceProps} />;
          }
          case 'Profession': {
            return <ChooseProfession {...choiceProps} />;
          }
          case 'Select': {
            return (
              <ChooseSelection
                {...choiceProps}
                selectionName={chooseTarget.data[1]}
              />
            );
          }
          case 'Slots':
          case 'NoviceSpellFrom':
          case 'ExpertSpellFrom':
          case 'MasterSpellFrom': {
            return <h1>TODO</h1>;
          }
        }
      })()}
    </ModalContainer>
  );
};

interface ModalContainerProps {
  children: React.ReactNode;
  onClose: () => void;
}

const ModalContainer = ({ children, onClose }: ModalContainerProps) => (
  <StaticCard
    className={cn(
      'p-10 w-fit max-w-300 bg-secondary-background text-foreground relative',
    )}
  >
    {children}
    <div
      className={cn(
        'w-6 h-6 absolute right-2 top-2 flex items-center justify-center rounded-full bg-inherit hover:brightness-90',
      )}
      onClick={onClose}
    >
      <X strokeWidth="1px" />
    </div>
  </StaticCard>
);
