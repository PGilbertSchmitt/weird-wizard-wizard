import { Palette } from 'lucide-react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '../ui/neo/dropdown-menu';
import { cn } from '@/lib/utils';
import { useContext } from 'react';
import { ThemeProviderContext } from '@/contexts/theme-context';
import { cardStyle, pressStyle } from '../ui/styles';

export const ThemeSelector = () => {
  const { setTheme } = useContext(ThemeProviderContext);

  return (
    <div className="fixed top-0 right-0 m-4 z-100">
      <DropdownMenu>
        <DropdownMenuTrigger
          className={cn(
            cardStyle,
            pressStyle,
            'p-2',
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
