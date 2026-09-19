import { useUpdateCharacterHealth } from '@/api/characters';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/neo/input';
import { cn } from '@/lib/utils';
import { clamp } from 'ramda';
import { useState } from 'react';

interface HealthAndDamageProps {
  characterId: number;
  maxHealth: number;
  curHealth: number;
  curDamage: number;
}

const parseAmount = (amount: string): number => {
  const parsedValue = parseInt(amount);
  return Number.isNaN(parsedValue) ? 0 : parsedValue;
};

export const HealthAndDamage = ({
  characterId,
  maxHealth,
  curHealth,
  curDamage,
}: HealthAndDamageProps) => {
  const [amount, setAmount] = useState('');
  const { mutateAsync: updateCharacterHealth } =
    useUpdateCharacterHealth(characterId);

  const updateAmount = (value: string) => {
    const parsedValue = parseInt(value);
    setAmount(
      Number.isNaN(parsedValue) ? '' : parsedValue.toString().slice(0, 4),
    );
  };

  const onUpdate = (newHealth: number, newDamage: number) => {
    const health = clamp(0, maxHealth, newHealth);
    const damage = clamp(0, health, newDamage);
    setAmount('');
    if (health !== curHealth || damage !== curDamage) {
      updateCharacterHealth({
        health,
        damage,
      });
    }
  };

  return (
    <div
      className={cn(
        'flex flex-col items-center border border-border w-fit p-4 gap-4',
      )}
    >
      <div className={cn('flex flex-row items-center text-center')}>
        <div className={cn('border border-border w-24 p-2 mr-2')}>
          <h3>Damage</h3>
          <p>
            {curDamage}/{curHealth}
          </p>
        </div>
        <div className={cn('border border-border w-24 p-2')}>
          <h3>Health</h3>
          <p>
            {curHealth}/{maxHealth}
          </p>
        </div>
      </div>
      <Input
        placeholder="amount"
        value={amount}
        onChange={(e) => updateAmount(e.target.value)}
      />
      <div className={cn('w-70')}>
        <div className={cn('flex flex-row')}>
          <Button
            pressStyle={false}
            onClick={() => onUpdate(curHealth, curDamage + parseAmount(amount))}
            className={cn(
              'rounded-none rounded-tl-base border-b border-t-2 border-r border-l-2 p-1 w-full',
            )}
          >
            Take Damage
          </Button>
          <Button
            pressStyle={false}
            onClick={() => onUpdate(curHealth, curDamage - parseAmount(amount))}
            className={cn(
              'rounded-none rounded-tr-base border-b border-t-2 border-l border-r-2 p-1 w-full',
            )}
          >
            Heal Damage
          </Button>
        </div>
        <div className={cn('flex flex-row')}>
          <Button
            pressStyle={false}
            onClick={() => onUpdate(curHealth + parseAmount(amount), curDamage)}
            className={cn(
              'rounded-none rounded-bl-base border-t border-b-2 border-r border-l-2 p-1 w-full',
            )}
          >
            Restore Health
          </Button>
          <Button
            pressStyle={false}
            onClick={() => onUpdate(curHealth - parseAmount(amount), curDamage)}
            className={cn(
              'rounded-none rounded-br-base border-t border-b-2 border-l border-r-2 p-1 w-full',
            )}
          >
            Lose Health
          </Button>
        </div>
      </div>
    </div>
  );
};
