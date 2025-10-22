import React from 'react';
import Nav from './nav';
import { cn } from '@/lib/utils';
import { ThemeSelector } from './theme-selector';

export const Layout = ({ children }: { children: React.ReactNode }) => {
  return (
    <>
      <Nav />
      <ThemeSelector />
      <main className={cn('w-fit m-auto min-h-screen')}>
        <div className="container">{children}</div>
      </main>
    </>
  );
};
