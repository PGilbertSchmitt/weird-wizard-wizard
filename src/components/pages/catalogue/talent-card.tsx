import { InfoTable } from "@/components/ui/info-table";
import { OptionBlock } from "@/components/ui/option-block";
import { cn } from "@/lib/utils";
import { TalentRestore } from "@/types/etc";
import { FullPathTalent } from "@/types/path";

interface TalentCardProps {
  talent: FullPathTalent;
}

export const TalentCard = ({ talent }: TalentCardProps) => {
  const restore = restoreString(talent.restore);
  return (
  <div className={cn("pl-4 p-2 my-2 border rounded-base")}>
    <h3>{talent.name}</h3>
    {talent.charges && (
      <p className="pl-4"><b>Charges:</b> {talent.charges}</p>
    )}

    {restore && (
      <p className="pl-4"><b>Recharges</b> {restore}</p>
    )}

    <p className="pl-4">{talent.description}</p>

    {talent.option_block && (
      <OptionBlock hideTitle block={talent.option_block} />
    )}

    {talent.info_table && (
      <InfoTable table={talent.info_table} />
    )}
  </div>
);
};

const restoreString = (restore: TalentRestore) => {
  switch (restore) {
    case "None": return null;
    case "LuckEnds": return "after Luck ends";
    case "Day": return "after 24 hours";
    case "Hour": return "after 1 hour";
    case "Minute": return "after 1 minute";
    case "Rest": return "after resting";
  }
}
