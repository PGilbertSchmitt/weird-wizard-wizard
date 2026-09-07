import { useAllAncestries } from '@/api/ancestries';
import { Button } from '@/components/ui/button';
import { StaticCard } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { useCollapseState } from '@/hooks/use-collapse-state';
import { cn } from '@/lib/utils';
import { AncestrySense, FullAncestry } from '@/types/path';
import {
  ChevronDown,
  ChevronUp,
  ListChevronsDownUp,
  ListChevronsUpDown,
} from 'lucide-react';
import { useMemo } from 'react';
import { FullSpeedTrait } from '@/types/other_info';
import { TalentCard } from './talent-card';

export const Ancestries = () => {
  const { isFetched, data: rawAncestries } = useAllAncestries();
  const ancestries = rawAncestries || [];

  const ids = useMemo(() => {
    if (isFetched) {
      return ancestries.map((a) => a.id);
    } else {
      return [];
    }
  }, [isFetched, ancestries]);

  const { isCollapsed, toggleCollapse, toggleAll, allCollapsed } =
    useCollapseState(ids);

  if (!isFetched) {
    return null;
  }

  return (
    <div className={cn('w-dvw max-w-250 px-4')}>
      <Button className="p-1" onClick={toggleAll}>
        {allCollapsed ? (
          <ListChevronsUpDown strokeWidth="1px" size="14px" />
        ) : (
          <ListChevronsDownUp strokeWidth="1px" size="14px" />
        )}
      </Button>

      {ancestries.map((item) => {
        const collapsed = isCollapsed(item.id);
        const attributes = calculateAttributes(item);

        return (
          <StaticCard key={item.id} className={cn('my-4 p-0')}>
            <div
              className="flex flex-row justify-between items-center p-2 cursor-pointer"
              onClick={() => toggleCollapse(item.id)}
            >
              <h2>{item.name}</h2>
              {collapsed ? (
                <ChevronDown strokeWidth="1px" size="18px" />
              ) : (
                <ChevronUp strokeWidth="1px" size="18px" />
              )}
            </div>

            {!collapsed && (
              <>
                <Separator />
                <div className={cn('bg-secondary-background text-foreground')}>
                  <div className="p-4 flex flex-row flex-wrap gap-x-5 gap-y-1">
                    {attributes.map((attr) => (
                      <p className="block">
                        <b>{attr.label}:</b> {attr.value}
                      </p>
                    ))}
                  </div>

                  <Separator />

                  <div className="py-1 px-4">
                    {item.talents.map((talent) => (
                      <TalentCard talent={talent} />
                    ))}
                  </div>
                </div>
              </>
            )}
          </StaticCard>
        );
      })}
    </div>
  );
};

interface Attribute {
  label: string;
  value: string;
}

const calculateAttributes = (ancestry: FullAncestry): Attribute[] => {
  const attrs: Attribute[] = [];

  const speedString = speedAttrString(ancestry.speed, ancestry.speed_traits);
  if (speedString) {
    attrs.push({
      label: 'Speed',
      value: speedString,
    });
  }

  attrs.push({
    label: 'Size',
    value: ancestry.size,
  });

  if (ancestry.add_health > 0) {
    attrs.push({
      label: 'Health',
      value: `+${ancestry.add_health}`,
    });
  }

  if (ancestry.add_nat_def > 0) {
    attrs.push({
      label: 'Natural Defense',
      value: `+${ancestry.add_nat_def}`,
    });
  }

  if (ancestry.descriptor) {
    attrs.push({
      label: 'Descriptor',
      value: ancestry.descriptor,
    });
  }

  if (ancestry.languages.length > 0) {
    attrs.push({
      label: 'Languages',
      value: ancestry.languages.map((l) => l.name).join(', '),
    });
  }

  const senseString = senseAttrString(ancestry.senses);
  if (senseString) {
    attrs.push({
      label: 'Senses',
      value: senseString,
    });
  }

  if (ancestry.immunities.length > 0) {
    attrs.push({
      label: 'Immunities',
      value: ancestry.immunities.map((i) => i.name).join(', '),
    });
  }

  return attrs;
};

// We can have a little duplication, as a treat
const speedTraitString = (traits: FullSpeedTrait[]) => {
  if (traits.length === 0) {
    return null;
  }

  const traitSubstrings = traits.map((trait) => {
    const amountString = trait.amount ? `${trait.amount} ${trait.unit}` : '';

    return `${trait.name} ${amountString}`.trim();
  });

  return traitSubstrings.join(', ');
};

const speedAttrString = (speed: number, traits: FullSpeedTrait[]) => {
  const traitString = speedTraitString(traits);

  if (traitString) {
    return `${speed} (${traitString})`;
  } else {
    return speed.toString();
  }
};

const senseAttrString = (senses: AncestrySense[]) => {
  if (senses.length === 0) {
    return null;
  }

  const senseSubstrings = senses.map((sense) => {
    const senseString = sense.amount ? `${sense.amount} ${sense.unit}` : '';
    return `${sense.name} ${senseString}`.trim();
  });

  return senseSubstrings.join(', ');
};
