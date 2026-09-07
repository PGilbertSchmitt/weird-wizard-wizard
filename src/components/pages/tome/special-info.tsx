import { cn } from '@/lib/utils';
import { isNil } from 'ramda';
import { useMemo } from 'react';

interface SpecialInfoProps {
  specialInfo: string | null;
}

export const SpecialInfo = ({ specialInfo }: SpecialInfoProps) => {
  const info = useMemo(() => {
    if (isNil(specialInfo)) {
      return null;
    }
    const elements = specialInfo.split('|').map((line) => line.trim());

    return elements.length > 1 ? elements : elements[0];
  }, [specialInfo]);

  if (isNil(info)) {
    return null;
  }

  if (typeof info === 'string') {
    return (
      <p
        className={cn(
          'border rounded-base bg-secondary-background text-foreground p-4',
        )}
      >
        {info}
      </p>
    );
  }

  return (
    <ul
      className={cn(
        'border rounded-base bg-secondary-background text-foreground p-4 pl-6 list-disc',
      )}
    >
      {info.map((item) => (
        <li key={item}>{item}</li>
      ))}
    </ul>
  );
};
