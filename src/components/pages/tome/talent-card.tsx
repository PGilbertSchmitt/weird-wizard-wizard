import { StaticCard } from "@/components/ui/card";
import { TalentItem } from "@/lib/types";

interface TalentCardProps {
  talent: TalentItem;
  onClick?: () => void;
}

export const TalentCard = ({ talent, onClick }: TalentCardProps) => (
  <div className="p-2">
    <StaticCard className="min-w-100" onClick={onClick}>
      <h2>{talent.name}</h2>
    </StaticCard>
  </div>
);