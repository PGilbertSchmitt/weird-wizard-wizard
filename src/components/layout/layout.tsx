import { useIsSeeded } from '@/api/seed';
import { cn } from '@/lib/utils';
import { Link, useLocation } from 'react-router';
import { User, WandSparkles, Swords, FileUp } from 'lucide-react';
import { ThemeSelector } from './theme-selector';

type IconType = typeof WandSparkles;

interface Tab {
  label: string;
  path: string;
  icon: IconType;
  seedRequired: boolean;
}

const TABS: Tab[] = [
  {
    label: 'Home',
    path: '/',
    icon: User,
    seedRequired: false,
  },
  {
    label: 'Tome',
    path: '/Tome',
    icon: WandSparkles,
    seedRequired: true,
  },
  {
    label: 'Catalogue',
    path: '/Catalogue',
    icon: Swords,
    seedRequired: true,
  },
  {
    label: 'Import',
    path: '/Import',
    icon: FileUp,
    seedRequired: false,
  },
];

export const Layout = ({ children }: { children: React.ReactNode }) => {
  const { pathname } = useLocation();
  const { isFetched, data: isSeeded } = useIsSeeded();

  if (!isFetched) {
    return (
      <div className="m-auto w-100 p-10">
        <p>
          If you can see this message for longer than 1 second, the app is
          probably broken.
        </p>
      </div>
    );
  }

  return (
    <main>
      <div
        className={cn(
          `w-full p-4 mb-5 mr-20 font-base flex flex-row justify-center`,
        )}
      >
        {TABS.map((tab) => {
          const active = !tab.seedRequired || isSeeded;
          return (
            <Link
              key={tab.label}
              to={active ? tab.path : '#'}
              className={cn(
                `relative bg-main  first:rounded-l-base last:rounded-r-base
                border-border border-l border-t-2 border-b-2 first:border-l-2 last:border-r-2
                px-8 pt-3 pb-2 transition-color flex items-center`,
                pathname === tab.path && 'brightness-80',
                active ? 'hover:brightness-80' : 'brightness-50',
              )}
            >
              <tab.icon
                size="14px"
                strokeWidth="1px"
                className={cn(`absolute left-4 top-1/2 -translate-1/2`)}
              />
              {tab.label}
            </Link>
          );
        })}
      </div>
      <ThemeSelector />
      <div className="w-fit h-fit pb-5 m-auto">{children}</div>
    </main>
  );
};
