import { cn } from '@/lib/utils';
import { Link, useLocation } from 'react-router';

const links = [
  {
    path: '/',
    text: 'Characters',
  },
  {
    path: '/seed',
    text: 'Seed',
  },
];

export default function Nav() {
  const { pathname } = useLocation();

  return (
    <div className="fixed top-5 left-5 z-50">
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
                `bg-main hover:brightness-90
                border-border border-l-2 border-r-2 first:border-t-2 last:border-b-2
                first:rounded-t-base last:rounded-b-base
                px-5 py-2 transition-color`,
                pathname === link.path && 'brightness-90',
              )}
              to={link.path}
            >
              {link.text}
            </Link>
          );
        })}
      </nav>
    </div>
  );
}
