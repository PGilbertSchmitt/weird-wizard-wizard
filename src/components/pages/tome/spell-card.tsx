import { StaticCard } from '@/components/ui/card';
import { Paragraph } from '@/components/ui/paragraph';
import { Separator } from '@/components/ui/separator';
import { SpellItem } from '@/lib/types';
import { AttributeRows, AttributeTable } from '@/components/ui/attribute-table';
import { useMemo } from 'react';
import { Waypoints } from 'lucide-react';
import { OptionBlock } from '@/components/ui/option-block';
import { InfoTable } from '@/components/ui/info-table';
import { Badge } from '@/components/ui/badge';

interface SpellCardProps {
  spell: SpellItem;
  onClick?: () => void;
}

export const SpellCard = ({ spell, onClick }: SpellCardProps) => {
  const attributeRows = useMemo(() => {
    console.log(spell);
    const durationString =
      `${spell.duration} ${spell.ritual ? '(Ritual)' : ''}`.trim();
    const staticAttributes: AttributeRows = [
      { label: 'CASTINGS', value: spell.castings },
      { label: 'DURATION', value: durationString },
      { label: 'TARGET', value: spell.target },
    ];

    if (spell.condition) {
      staticAttributes.push({ label: 'CONDITION', value: spell.condition });
    }
    return staticAttributes;
  }, [spell.id]);

  // The static card is wrapped in a padded div because if the card had the brick class,
  // the shadow would bleed over the column barrier
  return (
    <div className="brick p-2">
      <StaticCard className="min-w-50 text-sm" onClick={onClick}>
        <div className="flex justify-center gap-2  my-1">
          <h2 className="text-lg w-fit pt-1">{spell.name}</h2>
          {spell.ritual && (
            <Badge label={<Waypoints size="14px" strokeWidth="1px" />}>
              Ritual (takes 10 minutes to cast)
            </Badge>
          )}
        </div>
        <Separator />
        <AttributeTable rows={attributeRows} />
        <Separator />
        <Paragraph size="sm">{spell.description}</Paragraph>
        {spell.infoTable && (
          <>
            <Separator />
            <InfoTable table={spell.infoTable} />
          </>
        )}
        {spell.optionBlock && (
          <>
            <Separator />
            <OptionBlock block={spell.optionBlock} />
          </>
        )}
      </StaticCard>
    </div>
  );
};
