import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/neo/tooltip';

interface BadgeProps {
  children: React.ReactNode;
  label: React.ReactNode;
}

export const Badge = ({ children, label }: BadgeProps) => (
  <Tooltip>
    <TooltipTrigger asChild>
      <div className="border-border border-2 w-7 h-7 p-1 rounded-full flex items-center justify-center">
        {label}
      </div>
    </TooltipTrigger>
    <TooltipContent className="brightness-110">{children}</TooltipContent>
  </Tooltip>
);
