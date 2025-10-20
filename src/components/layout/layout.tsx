import React from 'react';
import Nav from './nav';
import { cn } from '@/lib/utils';
import { Button } from '../ui/button';
import { Palette } from 'lucide-react';

export const Layout = ({ children }: { children: React.ReactNode }) => {
  return (
    <>
      <div className="fixed top-0 right-0 m-5 z-100">
        <Button variant="default">
          <Palette strokeWidth="1px" />
        </Button>
      </div>
      <Nav />
      <div className={cn('w-fit m-auto min-h-screen')}>{children}</div>
    </>
  );
};
