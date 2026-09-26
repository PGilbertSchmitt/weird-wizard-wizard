import { useSpellsForTradition } from '@/api/magic';
import { SpellCard } from '@/components/pages/tome/magic-spell-card';
import { cn } from '@/lib/utils';
import { PathKind } from '@/types/etc';
import { FullSpell } from '@/types/magic';
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from '@/components/ui/neo/tabs';
import { useMemo } from 'react';
import { keys, sortBy } from 'ramda';

interface ChooseSpellsProps {
  traditionId: number;
  showTalentNote?: boolean;
  maxKind: PathKind;
  onSelect: (spellId: number, spellName: string) => void;
}

// This is specifically the form for selecting a novice spell when all magic talents
// have already been acquired from a tradition.
export const ChooseSpells = ({
  traditionId,
  onSelect,
  showTalentNote = false,
  maxKind,
}: ChooseSpellsProps) => {
  const { data: spells } = useSpellsForTradition(traditionId);

  const availableSpells = useMemo(() => {
    if (!spells) {
      return null;
    }
    const byKind = spells.reduce(
      (acc, spell: FullSpell) => {
        if (lessThanOrEqual(spell.path_kind, maxKind)) {
          acc[spell.path_kind] ||= [];
          acc[spell.path_kind].push(spell);
        }
        return acc;
      },
      {} as Record<PathKind, Array<FullSpell>>,
    );

    return byKind;
  }, [spells, maxKind]);

  if (availableSpells === null) {
    return null;
  }

  const traditionName = availableSpells.Novice[0].tradition_name;
  const sortedKinds = sortBy(kind => KindValues[kind], keys(availableSpells));

  return (
    <>
      <h1>Choose a spell from the {traditionName} tradition</h1>

      {showTalentNote && (
        <p>
          You have all {traditionName} talents, so you must take a Novice spell
          from this tradition instead
        </p>
      )}

      {maxKind === 'Novice' ? (
        <>
          <div className={cn('w-full max-w-400 mx-2 flex flex-col gap-4')}>
            {availableSpells.Novice.map((spell) => (
              <SpellCard key={spell.id} spell={spell} onSelect={onSelect} />
            ))}
          </div>
        </>
      ) : (
        <Tabs defaultValue={maxKind} className={cn('flex flex-col items-center')}>
          <TabsList>
            {sortedKinds.map(kind => (
              <TabsTrigger key={kind} value={kind}>{kind}</TabsTrigger>
            ))}
          </TabsList>
          {sortedKinds.map(kind => (
            <div className={cn('w-full max-w-400 mx-2 flex flex-col gap-4')}>
              <TabsContent key={kind} value={kind}>
                {availableSpells[kind].map((spell) => (
                  <SpellCard key={spell.id} spell={spell} onSelect={onSelect} />
                ))}
              </TabsContent>
            </div>
          ))}
        </Tabs>
      )}
    </>
  );
};

const KindValues: Record<PathKind, number> = {
  Novice: 1,
  Expert: 2,
  Master: 3,
};

const lessThanOrEqual = (kind: PathKind, limit: PathKind): boolean => {
  return KindValues[kind] <= KindValues[limit];
};
