import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/neo/tooltip';

interface DiskProps {
  children: React.ReactNode;
}

// A disk is like a badge, but just one neat little circle. Great for Icons

export const Disk = ({ children }: DiskProps) => (
  <div className="border-border border-2 w-7 h-7 p-1 rounded-full flex items-center justify-center">
    {children}
  </div>
);

interface DiskTooltipProps extends DiskProps {
  label: React.ReactNode;
}

export const DiskWithTooltip = ({ children, label }: DiskTooltipProps) => (
  <Tooltip>
    <TooltipTrigger>
      <Disk>{children}</Disk>
    </TooltipTrigger>
    <TooltipContent className="brightness-110">{label}</TooltipContent>
  </Tooltip>
);
