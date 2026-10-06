import { ClockArrowRight } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useModal } from "@/hooks/modal";
import { FullCharacter } from "@/types/character";

interface TimePasserProps {
  character: FullCharacter;
}

export const TimePasser = ({ character }: TimePasserProps) => {
  // const { mutateAsync: passTime } = usePassTime(characterId);
  const { pushModal } = useModal();

  return (
    <Button onClick={() => pushModal({
      type: 'Time',
      character,
    })}>
      <ClockArrowRight strokeWidth='1px' />
    </Button>
  );
};
