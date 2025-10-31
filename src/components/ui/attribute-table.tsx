import { cn } from "@/lib/utils";

interface AttributeTableProps {
  rows: Array<{ label: string; value: React.ReactNode; }>
}

export const AttributeTable = ({ rows }: AttributeTableProps) => (
  <table className="w-full">
    <tbody>
      {rows.map(row => (
        <tr key={row.label} className={cn('align-top border-b-1 border-border last:border-none')}>
          <th className={cn("p-0.5 pr-5 w-1 whitespace-nowrap")} align="left">{row.label}</th>
          <td className={cn("p-0.5")} align="left">{row.value}</td>
        </tr>
      ))}
    </tbody>
  </table>
);