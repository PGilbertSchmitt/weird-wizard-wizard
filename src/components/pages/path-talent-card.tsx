import { FullPathTalent } from '@/types/path';
import { useMemo } from 'react';
import { AttributeRows, AttributeTable } from '../ui/attribute-table';
import { StaticCard } from '../ui/card';
import { cn } from '@/lib/utils';
import { DiskWithTooltip } from '../ui/disk';
import { Sparkles } from 'lucide-react';
import { Separator } from '../ui/separator';
import { Paragraph } from '../ui/paragraph';
import { OptionBlock } from '../ui/option-block';
import { InfoTable } from '../ui/info-table';

interface PathTalentCardProps {
  talent: FullPathTalent;
}

export const PathTalentCard = ({ talent }: PathTalentCardProps) => {
  const attributeRows = useMemo(() => {
    const staticAttributes: AttributeRows = [];

    if (talent.activate) {
      staticAttributes.push({
        label: 'Activation:',
        value: talent.restore
          .split(',')
          .map((s) => s.trim())
          .join(', '),
      });
    }

    if (talent.charges) {
      staticAttributes.push({
        label: 'Charges:',
        value: talent.charges,
      });
    }

    staticAttributes.push({
      label: 'Restore:',
      value: talent.restore
        .split(',')
        .map((s) => s.trim())
        .join(', '),
    });

    return staticAttributes;
  }, [talent.id]);

  return (
    <StaticCard className="p-0">
      <div className={cn('p-2')}>
        <div className={cn('flex justify-start gap-5 my-1')}>
          <h2 className={cn('text-lg w-fit pt-1')}>{talent.name}</h2>
          <div className={cn('flex gap-2')}>
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
