import { NavCard } from "@/components/ui/card";
import { TraditionIndexItem } from "@/lib/types";
import { cn } from "@/lib/utils";

interface TraditionCardProps {
  tradition: TraditionIndexItem;
}

export const TraditionCard = ({ tradition }: TraditionCardProps) => {
  return (
    <NavCard
      className={cn('w-[calc(32%)] min-w-50 my-2')}
      href={`/tome/${tradition.id}`}
    >
      <h2>{tradition.name}</h2>
      <p>{tradition.blurb}</p>
    </NavCard>
  );
};
