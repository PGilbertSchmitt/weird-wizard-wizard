import { AttributeTable, AttributeRows } from "@/components/ui/attribute-table";
import { StaticCard } from "@/components/ui/card";
import { Paragraph } from "@/components/ui/paragraph";
import { Separator } from "@/components/ui/separator";
import { MagicTalentCharge, MagicTalentCharges, MagicTalentRestoration, MagicTalentRestorations } from "@/lib/db/enums";
import { TalentItem } from "@/lib/types";
import { useMemo } from "react";
import { Sparkles, Waypoints } from "lucide-react";
import { OptionBlock } from "@/components/ui/option-block";
import { InfoTable } from "@/components/ui/info-table";
import { Badge } from "@/components/ui/badge";

interface TalentCardProps {
  talent: TalentItem;
  onClick?: () => void;
}

export const TalentCard = ({ talent, onClick }: TalentCardProps) => {
  const attributeRows = useMemo(() => {
    const staticAttributes: AttributeRows = [
      { label: 'ACTIVATION', value: talent.activations.join(', ')},
    ];
    const chargesStr = charges(talent.charges);
    if (chargesStr) {
      staticAttributes.push({ label: 'CHARGES', value: chargesStr });
    }
    const restoreStr = restore(talent.restore);
    if (restoreStr) {
      staticAttributes.push({ label: 'RESTORE', value: restoreStr });
    }
    return staticAttributes;
  }, [talent.id]);

  // The static card is wrapped in a padded div because if the card had the brick class,
  // the shadow would bleed over the column barrier
  return (
    <div className="brick p-2">
      <StaticCard className="min-w-100" onClick={onClick}>
        <div className='flex justify-center gap-2 my-1'>
          <h2 className="text-lg w-fit pt-1">{talent.name}</h2>
          <Badge label={<Sparkles size='14px' strokeWidth="1px" />}>
            This talent is considered Magical
          </Badge>
          {talent.activations.includes('Ritual') && (
            <Badge label={<Waypoints size='14px' strokeWidth="1px" />}>
              Ritual (takes 10 minutes to cast)
            </Badge>
          )}
        </div>
        <Separator />
        <AttributeTable rows={attributeRows} />
        <Separator />
        <Paragraph size="sm">{talent.description}</Paragraph>
        {talent.infoTable && (
          <>
            <Separator />
            <InfoTable table={talent.infoTable} />
          </>
        )}
        {talent.optionBlock && (
          <>
            <Separator />
            <OptionBlock block={talent.optionBlock} />
          </>
        )}
      </StaticCard>
    </div>
  );
};

const charges = (value: MagicTalentCharge) => {
  switch (value) {
    case MagicTalentCharges.NONE: return null;
    case MagicTalentCharges.ONE: return '1';
    case MagicTalentCharges.ONE_TWO_THREE: return '1 at level 1, 2 at level 3, and 3 at level 7'
  }
};

const restore = (value: MagicTalentRestoration) => {
  switch (value) {
    case MagicTalentRestorations.NONE: return null;
    case MagicTalentRestorations.DAY: return '24 hours';
    case MagicTalentRestorations.HOUR: return '1 hour';
    case MagicTalentRestorations.MINUTE: return '1 minute';
    default: return value;
  }
};
