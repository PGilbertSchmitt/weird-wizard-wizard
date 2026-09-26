import { useMagicTalents } from '@/api/magic';
import { cn } from '@/lib/utils';
import { FullCharacter } from '@/types/character';
import { useMemo } from 'react';
import { ChooseSpells } from '../choose-spells';
import { MagicTalentCard } from '@/components/pages/tome/magic-talent-card';

interface ChooseItemProps {
  character: FullCharacter;
  idxStr: string;
  traditionId: number;
  selectedTalentIds: number[];
  onSelect: (
    itemId: number,
    itemName: string,
    kind: 'talent' | 'spell',
  ) => void;
}

export const ChooseItem = ({
  character,
  idxStr,
  traditionId,
  selectedTalentIds,
  onSelect,
}: ChooseItemProps) => {
  const { data: talents } = useMagicTalents(traditionId);

  const remainingTalents = useMemo(() => {
    if (!talents) {
      return null;
    }

    const priorOwnedTalentIds = character.magic_talents
      .filter((talent) => talent[0].tradition_id === traditionId)
      .map((talent) => talent[0].id);

    const ownedTalentIds = [...priorOwnedTalentIds, ...selectedTalentIds];

    return talents.filter((t) => !ownedTalentIds.includes(t.id));
  }, [talents]);

  if (remainingTalents === null) {
    return null;
  }

  if (remainingTalents.length === 0) {
    return (
      <ChooseSpells
        traditionId={traditionId}
        maxKind='Novice'
        showTalentNote
        onSelect={(spellId, spellName) => onSelect(spellId, spellName, 'spell')}
      />
    );
  }

  // Pick talent
  return (
    <div>
      <h1>Pick {idxStr} magic talent</h1>

      <div className={cn('w-full max-w-400 mx-2')}>
        {remainingTalents.map((talent) => (
          <MagicTalentCard
            key={talent.id}
            talent={talent}
            onSelect={() => onSelect(talent.id, talent.name, 'talent')}
          />
        ))}
      </div>
    </div>
  );
};
