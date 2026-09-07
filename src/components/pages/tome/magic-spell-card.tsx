import { AttributeRows, AttributeTable } from '@/components/ui/attribute-table';
import { Badge } from '@/components/ui/badge';
import { StaticCard } from '@/components/ui/card';
import { InfoTable } from '@/components/ui/info-table';
import { OptionBlock } from '@/components/ui/option-block';
import { Paragraph } from '@/components/ui/paragraph';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { FullSpell } from '@/types/magic';
import { Waypoints } from 'lucide-react';
import { useMemo } from 'react';

interface SpellCardProps {
  spell: FullSpell;
}

export const SpellCard = ({ spell }: SpellCardProps) => {
  const attributeRows: AttributeRows = useMemo(() => {
    return [
      {
        label: 'Castings:',
        value: spell.castings,
      },
      {
        label: 'Duration:',
        value: spell.duration,
      },
      {
        label: 'Target:',
        value: spell.target,
      },
    ];
  }, [spell.id]);

  console.log('Spell', spell);

  return (
    <StaticCard className="p-0">
      <div className="p-2">
        <div className={cn('flex justify-between gap-2 my-1')}>
          <h2 className={cn('text-lg w-fit pt-1')}>{spell.name}</h2>
          <div className={cn('flex gap-2')}>
            {spell.ritual && (
              <Badge label="Ritual - Takes 10 minutes to cast">
                <Waypoints size="14px" strokeWidth="1px" />
              </Badge>
            )}
          </div>
        </div>

        <Separator />

        <AttributeTable rows={attributeRows} />
      </div>

      <Separator />

      <div className="bg-secondary-background text-foreground p-4">
        <Paragraph size="sm">{spell.description}</Paragraph>
        
        {spell.option_block && (
          <>
            <Separator />
            <OptionBlock block={spell.option_block} />
          </>
        )}

        {spell.info_table && (
          <>
            <Separator />
            <InfoTable table={spell.info_table} />
          </>
        )}
      </div>
    </StaticCard>
  );
};
