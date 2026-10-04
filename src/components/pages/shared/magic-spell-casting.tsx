import { useCastSpell, useSpellSlots } from '@/api/characters';
import { Button } from '@/components/ui/button';
import { Spinner } from '@/components/ui/neo/spinner';
import { cn } from '@/lib/utils';
import { FullCharacter } from '@/types/character';
import { FullSpell } from '@/types/magic';
import { times } from 'ramda';
import { useMemo } from 'react';

interface MagicSpellCastingProps {
  character: FullCharacter;
  spell: FullSpell;
}

export const MagicSpellCasting = ({
  character,
  spell,
}: MagicSpellCastingProps) => {
  const { data: spellSlotInfo } = useSpellSlots(character.id, spell.id);
  const { mutateAsync: castSpell } = useCastSpell(character.id);

  const [permCount, tempCount] = useMemo(() => {
    const ownedSpellCount = character.spells.filter(
      (s) => s[0].id === spell.id,
    ).length;
    const permCastings = ownedSpellCount * spell.castings;

    // I don't have any data on when a modifier is applied. Because of that, it's possible
    // for slot modifiers to arrive in any arbitrary order, and there's no way to know which
    // should be applied first. In order to keep the castings consistent regardless of the
    // order, I ensure all multiplications happen first, then any additions. I don't know
    // if there are specific rules about this, but it's also probably a very rare case, so
    // it's not that important. It would be too much work to make this work better for now.

    const [multiplier, add] = character.modified_slots.reduce(
      (acc, slotMod) => {
        if ('Times' in slotMod && slotMod.Times.spell_id === spell.id) {
          return [acc[0] * slotMod.Times.amount, acc[1]];
        } else if ('Plus' in slotMod && slotMod.Plus.spell_id === spell.id) {
          return [acc[0], acc[1] + slotMod.Plus.amount];
        } else {
          return acc;
        }
      },
      [1, 0] as [number, number],
    );

    const totalCastings = (permCastings + add) * multiplier;
    const extraCastings = totalCastings - permCastings;

    return [permCastings, extraCastings];
  }, [character]);

  // undefined means the request is loading, null means no data is saved
  if (spellSlotInfo === undefined) {
    return <Spinner />;
  }

  const permUsed = spellSlotInfo?.perm_used || 0;
  const tempUsed = spellSlotInfo?.temp_used || 0;

  const castable = permUsed < permCount || tempUsed < tempCount;

  return (
    <div className={cn('flex flex-row items-center gap-10')}>
      <Button
        className={cn('p-1')}
        disabled={!castable}
        onClick={() => {
          castSpell({
            spellId: spell.id,
            permCount,
            tempCount,
          });
        }}
      >
        Cast me
      </Button>

      <div className={cn('flex flex-row items-center gap-2')}>
        {times(
          (idx) => (idx < permUsed ? <SpellSlot filled /> : <SpellSlot />),
          permCount,
        )}

        {times(
          (idx) =>
            idx < tempUsed ? (
              <SpellSlot temporary filled />
            ) : (
              <SpellSlot temporary />
            ),
          tempCount,
        )}
      </div>
    </div>
  );
};

interface SpellSlotProps {
  filled?: boolean;
  temporary?: boolean;
}

export const SpellSlot = ({
  filled = false,
  temporary = false,
}: SpellSlotProps) => (
  <div
    className={cn(
      'border-2 border-border w-5 h-5',
      filled ? 'bg-main' : 'bg-secondary-background',
      temporary && 'rounded-full',
    )}
  />
);
