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

interface CharacterSpellsProps {
  spells: Array<FullSpell>
}

export const CharacterSpells = ({ spells }: CharacterSpellsProps) => {
  const spellsByKind = useMemo(() => {
    return spells.reduce((acc, spell) => {
      acc[spell.path_kind] ||= [];
      acc[spell.path_kind].push(spell);
      return acc;
    }, {} as Record<PathKind, Array<FullSpell>>);
  }, [spells]);

  const ownedKinds = (['Novice', 'Expert', 'Master'] as PathKind[]).filter(kind => kind in spellsByKind);
  
  if (ownedKinds.length < 2) {
    <div className={cn('flex flex-col gap-4')}>
      {spellsByKind[ownedKinds[0]].map((spell) => (
        <SpellCard
          key={spell.id}
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
            {spellsByKind[kind].map((spell) => (
              <SpellCard
                key={spell.id}
                spell={spell}
              />
            ))}
          </div>
        </TabsContent>
      ))}
    </Tabs>
  );
};
