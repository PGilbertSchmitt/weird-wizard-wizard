import { cn } from '../lib/utils';
import { Link } from 'react-router';

export default function Nav() {
  const path = window.location.pathname;

  const links = [
    {
      path: '/',
      text: 'Home',
    },
    {
      path: '/seed',
      text: 'Seed',
    },
    {
      path: '/themes',
      text: 'Themes',
    },
  ];

  return (
    <div className="fixed top-5 left-0 z-50 w-full">
      <nav className="text-main-foreground border-border shadow-shadow rounded-base bg-main font-base w450:gap-4 mx-auto flex w-max gap-5 border-2 p-2.5 px-5 text-sm sm:text-base">
        {links.map((link) => {
          console.log(link.path, '===', path, '=>', link.path === path);
          return (
            <Link
              key={link.path}
              className={cn(
                'hover:border-border rounded-base border-2 px-2 py-1 transition-colors',
                path === link.path ? 'border-border' : 'border-none',
              )}
              to={link.path}
            >
              {link.text}
            </Link>
          );
        })}
        {/* <ThemeSwitcher /> */}
      </nav>
    </div>
  );
}
