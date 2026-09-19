import { Minus, Plus } from 'lucide-react';
import { cn } from '@/lib/utils';

interface ControlledCounterProps {
  value: number;
  range: [number, number];
  onAdd: () => void;
  onSubtract: () => void;
}

export const ControlledCounter = ({
  value,
  onAdd,
  onSubtract,
  range,
}: ControlledCounterProps) => {
  const [min, max] = range;
  const subtractDisabled = value <= min;
  const addDisabled = value >= max;

  return (
    <div className={cn('flex flex-row items-center text-main-foreground')}>
      <button
        onClick={onSubtract}
        disabled={subtractDisabled}
        className={cn(
          'rounded-l-base bg-main h-8 w-8 border-2 flex flex-row justify-center items-center cursor-pointer',
          subtractDisabled && 'brightness-50 cursor-default',
        )}
      >
        <Minus strokeWidth="1px" size="16px" />
      </button>
      <div
        className={cn(
          'bg-white shadow p-2 h-8 w-8 border-y-2 flex flex-row justify-center items-center',
        )}
      >
        {value}
      </div>
      <button
        onClick={onAdd}
        disabled={addDisabled}
        className={cn(
          'rounded-r-base bg-main h-8 w-8 border-2 flex flex-row justify-center items-center cursor-pointer',
          addDisabled && 'brightness-50 cursor-default',
        )}
      >
        <Plus strokeWidth="1px" size="16px" />
      </button>
    </div>
  );
};
