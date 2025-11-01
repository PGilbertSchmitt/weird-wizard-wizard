import { InfoTable as InfoTableType } from "@/lib/types";
import { cn } from "@/lib/utils";

interface InfoTableProps {
  table: InfoTableType;
}

export const InfoTable = ({ table }: InfoTableProps) => (
  <div className="my-2">
    <h3>{table.name}</h3>
    <table>
      <thead>
        <tr>
          <th
            className={cn('p-0.5 pr-5 w-1 whitespace-nowrap')}
            align="right"
          >
            {table.keyLabel}
          </th>
          <th align="left">{table.valueLabel}</th>
        </tr>
      </thead>
      <tbody>
        {table.rows.map((row) => (
          <tr
            key={row.key}
            className={cn(
              'align-top border-b-1 border-border last:border-none',
            )}
          >
            <th
              className={cn('p-0.5 pr-5 w-1 whitespace-nowrap')}
              align="right"
            >
              {row.key}
            </th>
            <td className={cn('p-0.5')} align="left">
              {row.value}
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  </div>
);
