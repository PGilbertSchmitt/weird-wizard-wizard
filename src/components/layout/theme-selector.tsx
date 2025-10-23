import { Palette } from 'lucide-react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '../ui/dropdown-menu';
import { cn } from '@/lib/utils';
import { useContext } from 'react';
import { ThemeProviderContext } from '@/contexts/theme-context';

export const ThemeSelector = () => {
  const { setTheme } = useContext(ThemeProviderContext);

  return (
    <div className="fixed top-0 right-0 m-5 z-100">
      <DropdownMenu>
        <DropdownMenuTrigger
          className={cn(
            `bg-main border-border border-2 shadow-shadow rounded-base p-1 text-main-foreground
            hover:translate-x-boxShadowX hover:translate-y-boxShadowY hover:shadow-none transition-all`,
          )}
        >
          <Palette strokeWidth="1px" />
        </DropdownMenuTrigger>
        <DropdownMenuContent className={cn('m-4')}>
          <DropdownMenuGroup>
            <DropdownMenuItem onSelect={() => setTheme('dark-red')}>
              Dark Red
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={() => setTheme('light-red')}>
              Light Red
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={() => setTheme('dark-cyan')}>
              Dark Cyan
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={() => setTheme('light-cyan')}>
              Light Cyan
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={() => setTheme('dark-amber')}>
              Dark Amber
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={() => setTheme('light-amber')}>
              Light Amber
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
};
