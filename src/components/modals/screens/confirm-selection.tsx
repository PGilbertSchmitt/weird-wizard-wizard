import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { FullChoiceModifier } from '../type';
import { useModal } from '@/hooks/modal';
import { useSaveChoice } from '@/api/characters';

interface ConfirmSelectionProps {
  characterId: number;
  children: React.ReactNode;
  modifier: FullChoiceModifier;
  values: string[];
}

export const ConfirmSelection = ({
  characterId,
  children,
  modifier,
  values,
}: ConfirmSelectionProps) => {
  const { mutateAsync: saveChoice } = useSaveChoice(characterId);
  const { popNonErrorModal, pushError } = useModal();

  return (
    <div className={cn('w-40')}>
      <h3 className="mb-4">Selections:</h3>

      {children}

      <Button
        className={cn('w-full mt-4')}
        onClick={() => {
          saveChoice({
            modifier,
            values,
          })
            .then(popNonErrorModal)
            .catch((err) => {
              popNonErrorModal();
              pushError(err.toString());
            });
        }}
      >
        Confirm
      </Button>
    </div>
  );
};
