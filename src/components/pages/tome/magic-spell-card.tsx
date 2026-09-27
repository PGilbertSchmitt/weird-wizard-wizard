import { AttributeRows, AttributeTable } from '@/components/ui/attribute-table';
import { DiskWithTooltip } from '@/components/ui/disk';
import { StaticCard } from '@/components/ui/card';
import { InfoTable } from '@/components/ui/info-table';
import { OptionBlock } from '@/components/ui/option-block';
import { Paragraph } from '@/components/ui/paragraph';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { FullSpell } from '@/types/magic';
import { Waypoints } from 'lucide-react';
import { useMemo } from 'react';
import { Button } from '@/components/ui/button';

interface SpellCardProps {
  spell: FullSpell;
  count?: number;
  onSelect?: (id: number, name: string) => void;
}

export const SpellCard = ({ spell, onSelect, count }: SpellCardProps) => {
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

  return (
    <StaticCard className="p-0">
      <div className="p-2">
        <div className={cn('flex justify-start gap-5 my-1')}>
          {onSelect && (
            <Button
              pressStyle={false}
              className={cn('px-5 py-1')}
              onClick={() => onSelect(spell.id, spell.name)}
            >
              Pick
            </Button>
          )}
          <h2 className={cn('text-lg w-fit pt-1')}>
            {spell.name} {count && `x${count}`}
          </h2>
          <div className={cn('flex gap-2')}>
            {spell.ritual && (
              <DiskWithTooltip label="Ritual - Takes 10 minutes to cast">
                <Waypoints size="14px" strokeWidth="1px" />
              </DiskWithTooltip>
            )}
          </div>
        </div>

        <Separator />

        <AttributeTable rows={attributeRows} />
      </div>

      <Separator />

      <div className="bg-secondary-background text-foreground rounded-b-base p-4">
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
