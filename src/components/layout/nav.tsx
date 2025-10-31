import { cn } from '@/lib/utils';
import { Link, useLocation } from 'react-router';
import { Home, FileUp, WandSparkles, Swords, ArrowBigLeft } from 'lucide-react';
import { init } from 'ramda';
import { cardStyle, pressStyle } from '../ui/styles';

const links = [
  {
    path: '/',
    text: 'Characters',
    icon: Home,
  },
  {
    path: '/tome',
    text: 'Magic Tome',
    icon: WandSparkles,
  },
  {
    path: '/paths',
    text: 'All Paths',
    icon: Swords,
  },
  {
    // Should include some conditional formatting when no imports have been run
    path: '/seed',
    text: 'Import',
    icon: FileUp,
  },
];

export default function Nav() {
  const { pathname } = useLocation();
  const headlessPath = pathname.charAt(0) === '/'
    ? pathname.slice(1)
    : pathname;
  const paths = headlessPath.split('/');
  const backTo = paths.length > 1 ? `/${init(paths).join('/')}` : null;

  return (
    <div className="fixed top-4 left-4 z-50 w-55">
      <nav
        className={cn(
          'flex flex-col text-main-foreground font-base border-border shadow-shadow rounded-base w-fit',
        )}
      >
        {links.map((link) => {
          return (
            <Link
              key={link.path}
              className={cn(
                `relative bg-main hover:brightness-90
                border-border border-l-2 border-r-2 first:border-t-2 last:border-b-2
                first:rounded-t-base last:rounded-b-base
                px-8 pt-3 pb-2 transition-color flex items-center`,
                pathname === link.path && 'brightness-80',
              )}
              to={link.path}
            >
              <link.icon
                size="14px"
                strokeWidth="1px"
                className={cn(`absolute left-4 top-1/2 -translate-1/2`)}
              />
              <span>{link.text}</span>
            </Link>
          );
        })}
      </nav>
      {backTo && (
        <Link
          to={backTo}
          className={cn(
            cardStyle,
            pressStyle,
            'absolute right-0 top-0 p-2',
          )}
        >
          <ArrowBigLeft strokeWidth="1px" />
        </Link>
      )}
    </div>
  );
}
