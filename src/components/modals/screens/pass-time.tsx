import { usePassTime } from '@/api/characters';
import { Button } from '@/components/ui/button';
import { useModal } from '@/hooks/modal';
import { cn } from '@/lib/utils';
import { FullCharacter } from '@/types/character';
import { WhenDuration } from '@/types/modifiers';

interface PassTimeModalProps {
  character: FullCharacter;
}

export const PassTimeModal = ({ character }: PassTimeModalProps) => {
  const { popNonErrorModal, pushError } = useModal();
  const { mutateAsync: passTime } = usePassTime(character.id);

  const timeHandler = (type: WhenDuration['type']) => () => {
    passTime({ type })
      .then(popNonErrorModal)
      .catch((err) => {
        popNonErrorModal();
        pushError(err.toString());
      });
  };

  return (
    <div className={cn('flex flex-col items-center gap-5')}>
      <h2>Pass Time</h2>
      <div className={cn('flex justify-between gap-5')}>
        <Button className={cn('p-2')} onClick={timeHandler('Rest')}>
          Rest
        </Button>
        <Button className={cn('p-2')} onClick={timeHandler('OneMinute')}>
          Pass 1 Minute
        </Button>
      </div>
    </div>
  );
};
