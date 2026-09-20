import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/neo/tooltip';

interface BadgeProps {
  children: React.ReactNode;
}

export const Badge = ({ children }: BadgeProps) => (
  <div className="border-border border-2 p-1 rounded-full flex items-center justify-center">
    {children}
  </div>
);

interface BadgeTooltipProps extends BadgeProps {
  label: React.ReactNode;
}

export const BadgeWithTooltip = ({ children, label }: BadgeTooltipProps) => (
  <Tooltip>
    <TooltipTrigger>
      <Badge>{children}</Badge>
    </TooltipTrigger>
    <TooltipContent className="brightness-110">{label}</TooltipContent>
  </Tooltip>
);
