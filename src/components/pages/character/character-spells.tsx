import { PathKind } from "@/types/etc";
import { FullSpell } from "@/types/magic";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from '@/components/ui/neo/tabs';
import { useMemo } from "react";
import { cn } from "@/lib/utils";
import { SpellCard } from "../tome/magic-spell-card";
import { fromPairs, toPairs, values } from "ramda";

interface CharacterSpellsProps {
  spells: Array<FullSpell>
}

export const CharacterSpells = ({ spells }: CharacterSpellsProps) => {
  const spellsByKind = useMemo(() => {
    const byKinds = spells.reduce((acc, spell) => {
      acc[spell.path_kind] ||= [];
      acc[spell.path_kind].push(spell);
      return acc;
    }, {} as Record<PathKind, Array<FullSpell>>);

    return fromPairs(toPairs(byKinds).map(([kind, spells]) => {
      return [kind, collapseSpells(spells)] as const;
    }));
  }, [spells]);

  const ownedKinds = (['Novice', 'Expert', 'Master'] as PathKind[]).filter(kind => kind in spellsByKind);
  
  if (ownedKinds.length === 0) {
    return <p>No spells.</p>;
  }
  
  if (ownedKinds.length === 1) {
    <div className={cn('flex flex-col gap-4')}>
      {spellsByKind[ownedKinds[0]].map(([spell, count]) => (
        <SpellCard
          key={spell.id}
          count={count}
          spell={spell}
        />
      ))}
    </div>
  }  
  
  const defaultValue = ownedKinds[0];
  return (
    <Tabs defaultValue={defaultValue} className={cn('flex flex-col items-center')}>
      <TabsList>
        {ownedKinds.map(kind => (
          <TabsTrigger key={kind} value={kind}>
            {kind}
          </TabsTrigger>
        ))}
      </TabsList>
      {ownedKinds.map(kind => (
        <TabsContent key={kind} value={kind}>
          <div className={cn('flex flex-col gap-4')}>
            {spellsByKind[kind].map(([spell, count]) => (
              <SpellCard
                key={spell.id}
                count={count > 1 ? count : undefined}
                spell={spell}
              />
            ))}
          </div>
        </TabsContent>
      ))}
    </Tabs>
  );
};

const collapseSpells = (spells: Array<FullSpell>): Array<[FullSpell, number]> => {
  const output: Record<number, [FullSpell, number]> = [];

  for (const spell of spells) {
    if (output[spell.id]) {
      output[spell.id][1]++;
    } else {
      output[spell.id] = [spell, 1];
    }
  }

  return values(output);
};
