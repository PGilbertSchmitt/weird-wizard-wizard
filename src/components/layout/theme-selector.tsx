import { Palette, MoonIcon, SunIcon } from 'lucide-react';
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

const triggerStyle =
  'bg-main border-border border-t-2 border-b-2 p-2 transition-color cursor-pointer hover:brightness-90';

export const ThemeSelector = () => {
  const { light, setLight, setColor } = useContext(ThemeProviderContext);

  const isLight = light === 'light';

  return (
    <div className="fixed top-0 right-0 m-4 z-100 flex text-main-foreground">
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <div
            className={cn(triggerStyle, 'rounded-l-base border-r border-l-2')}
          >
            <Palette strokeWidth="1px" />
          </div>
        </DropdownMenuTrigger>
        <DropdownMenuContent className={cn('m-4')}>
          <DropdownMenuGroup>
            <DropdownMenuItem onSelect={() => setColor('red')}>
              Red
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={() => setColor('cyan')}>
              Cyan
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={() => setColor('amber')}>
              Amber
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
      <div
        className={cn(triggerStyle, 'rounded-r-base border-r-2')}
        onClick={() => {
          setLight(isLight ? 'dark' : 'light');
        }}
      >
        {isLight ? (
          <MoonIcon strokeWidth="1px" />
        ) : (
          <SunIcon strokeWidth="1px" />
        )}
      </div>
    </div>
  );
};
