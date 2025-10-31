import { StaticCard } from "@/components/ui/card";
import { Paragraph } from "@/components/ui/paragraph";
import { Separator } from "@/components/ui/separator";
import { SpellItem } from "@/lib/types";
import { AttributeTable } from "@/components/ui/attribute-table";

interface SpellCardProps {
  spell: SpellItem;
  onClick?: () => void;
}

export const SpellCard = ({ spell, onClick }: SpellCardProps) => (
  <div className="brick p-2">
    <StaticCard className="min-w-50 text-sm" onClick={onClick}>
      <h2 className="text-lg">{spell.name}</h2>
      <Separator />
      <AttributeTable
        rows={[
          { label: 'CASTINGS', value: spell.castings },
          { label: 'DURATION', value: spell.duration },
          { label: 'TARGET', value: spell.target },
        ]}
      />
      <Separator />
      <Paragraph size="sm">{spell.description}</Paragraph>
    </StaticCard>
  </div>
);
