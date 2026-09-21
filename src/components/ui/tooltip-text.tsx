import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/neo/tooltip';

interface TooltipTextProps {
  children: React.ReactNode;
  label: React.ReactNode;
}

export const TooltipText = ({
  children,
  label,
}: TooltipTextProps) => (
  <Tooltip delayDuration={1000}>
    <TooltipTrigger>
      <span className="underline">{children}</span>
    </TooltipTrigger>
    <TooltipContent className="brightness-110">{label}</TooltipContent>
  </Tooltip>
);
