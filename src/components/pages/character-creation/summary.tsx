import { Button } from '@/components/ui/button';
import { StaticCard } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import { Scores } from './choose-scores';
import { toPairs } from 'ramda';

interface SummaryProps {
  name: string;
  professionName: string;
  novicePathName: string;
  ancestryName: string;
  ancestryLocked: boolean;
  scores: Scores;
  onConfirm: () => void;
}

export const Summary = ({
  name,
  professionName,
  novicePathName,
  ancestryName,
  ancestryLocked,
  scores,
  onConfirm,
}: SummaryProps) => (
  <div className="flex flex-col align-middle">
    <StaticCard className={cn('my-4 p-0')}>
      <div className="flex flex-row justify-center items-center p-2 cursor-pointer">
        <h2>{name}</h2>
      </div>

      <Separator />

      <div className={cn('bg-secondary-background text-foreground')}>
        <table className="w-full">
          <tbody>
            <tr>
              <th className="p-2 text-right">Profession</th>
              <td className="p-2">{professionName}</td>
            </tr>
            <tr className="border-t">
              <th className="p-2 text-right">Ancestry</th>
              <td className="p-2">
                {ancestryName} {ancestryLocked && '(determined by novice path)'}
              </td>
            </tr>
            <tr className="border-t">
              <th className="p-2 text-right">Novice Path</th>
              <td className="p-2">{novicePathName}</td>
            </tr>
          </tbody>
        </table>

        <Separator />

        <div className={cn('flex flex-row justify-around p-2')}>
          {toPairs(scores).map(([ability, value]) => (
            <div key={ability} className={cn('flex flex-col items-center')}>
              <b>{fixCase(ability)}</b>
              <p>{value}</p>
            </div>
          ))}
        </div>
      </div>
    </StaticCard>
    <Button onClick={onConfirm}>Confirm Selection</Button>
  </div>
);

const fixCase = (s: string) => `${s[0].toUpperCase()}${s.slice(1)}`;
