import { cn } from '@/lib/utils';
import { ModalData, ModalTypes } from './type';
import { ChooseScores } from './screens/choose-scores';
import { StaticCard } from '../ui/card';
import { WithCharacter } from '../providers/character-provider';

interface ModalSwitchProps {
  modalData: ModalData;
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
          switch (chooseTarget.type) {
            case 'Score': {
              return (
                <ChooseScores
                  character={character}
                  keys={keys}
                  source={modalData.source}
                  modifier={modalData.modifier}
                />
              );
            }
            case 'Tradition': {
              return <h2>Pick a tradition</h2>;
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
