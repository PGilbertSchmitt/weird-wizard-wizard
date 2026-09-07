import { AttributeRows, AttributeTable } from '@/components/ui/attribute-table';
import { Badge } from '@/components/ui/badge';
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

interface MagicTalentCardProps {
  talent: FullMagicTalent;
}

export const MagicTalentCard = ({ talent }: MagicTalentCardProps) => {
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
        <div className={cn('flex justify-between gap-2 my-1')}>
          <h2 className={cn('text-lg w-fit pt-1')}>{talent.name}</h2>
          <div className={cn('flex gap-2')}>
            {talent.activate.includes('Ritual') && (
              <Badge label="Ritual - Takes 10 minutes to cast">
                <Waypoints size="14px" strokeWidth="1px" />
              </Badge>
            )}
            <Badge label="This talent is considered Magical">
              <Sparkles size="14px" strokeWidth="1px" />
            </Badge>
          </div>
        </div>

        <Separator />

        <AttributeTable rows={attributeRows} />
      </div>

      <Separator />

      <div className="bg-secondary-background text-foreground p-4">
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
