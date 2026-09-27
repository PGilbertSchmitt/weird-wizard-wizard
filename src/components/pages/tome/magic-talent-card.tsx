import { AttributeRows, AttributeTable } from '@/components/ui/attribute-table';
import { DiskWithTooltip } from '@/components/ui/disk';
import { StaticCard } from '@/components/ui/card';
import { InfoTable } from '@/components/ui/info-table';
import { OptionBlock } from '@/components/ui/option-block';
import { Paragraph } from '@/components/ui/paragraph';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { TalentRestore } from '@/types/etc';
import { FullMagicTalent, MagicTalentCharges } from '@/types/magic';
import { Sparkles, Waypoints } from 'lucide-react';
import { useMemo } from 'react';
import { Button } from '@/components/ui/button';

interface MagicTalentCardProps {
  talent: FullMagicTalent;
  onSelect?: (id: number, name: string) => void;
}

export const MagicTalentCard = ({ talent, onSelect }: MagicTalentCardProps) => {
  const attributeRows = useMemo(() => {
    const staticAttributes: AttributeRows = [
      {
        label: 'Activation:',
        value: talent.activate
          .split(',')
          .map((s) => s.trim())
          .join(', '),
      },
    ];

    const chargeString = charges(talent.charges);
    if (chargeString !== null) {
      staticAttributes.push({
        label: 'Charges:',
        value: chargeString,
      });
    }

    const restoreString = restore(talent.restore);
    if (restoreString !== null) {
      staticAttributes.push({
        label: 'Restore:',
        value: restoreString,
      });
    }

    return staticAttributes;
  }, [talent.id]);

  return (
    <StaticCard className="p-0">
      <div className={cn('p-2')}>
        <div className={cn('flex justify-start gap-5 my-1')}>
          {onSelect && (
            <Button
              pressStyle={false}
              className={cn('px-5 py-1')}
              onClick={() => onSelect(talent.id, talent.name)}
            >
              Pick
            </Button>
          )}
          <h2 className={cn('text-lg w-fit pt-1')}>{talent.name}</h2>
          <div className={cn('flex gap-2')}>
            {talent.activate.includes('Ritual') && (
              <DiskWithTooltip label="Ritual - Takes 10 minutes to cast">
                <Waypoints size="14px" strokeWidth="1px" />
              </DiskWithTooltip>
            )}
            <DiskWithTooltip label="This talent is considered Magical">
              <Sparkles size="14px" strokeWidth="1px" />
            </DiskWithTooltip>
          </div>
        </div>

        <Separator />

        <AttributeTable rows={attributeRows} />
      </div>

      <Separator />

      <div className="bg-secondary-background text-foreground p-4 rounded-b-base">
        <Paragraph size="sm">{talent.description}</Paragraph>

        {talent.option_block && (
          <>
            <Separator />
            <OptionBlock block={talent.option_block} />
          </>
        )}

        {talent.info_table && (
          <>
            <Separator />
            <InfoTable table={talent.info_table} />
          </>
        )}
      </div>
    </StaticCard>
  );
};

const charges = (value: MagicTalentCharges) => {
  switch (value) {
    case 'None':
      return null;
    case 'One':
      return '1';
    case 'OneTwoThree':
      return '1 at level 1 / 2 at level 3+ / 3 at level 7+';
  }
};

const restore = (value: TalentRestore) => {
  switch (value) {
    case 'None':
      return null;
    case 'Minute':
      return '1 minute';
    case 'Hour':
      return '1 hour';
    case 'Day':
      return '24 hours';
    case 'LuckEnds':
      return 'Luck ends';
    case 'Rest':
      return 'On rest';
  }
};
