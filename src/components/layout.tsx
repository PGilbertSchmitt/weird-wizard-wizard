import React from 'react';
import Nav from './nav';

export const Layout = ({ children }: { children: React.ReactNode }) => (
  <div className="text-foreground mx-auto w-[750px] max-w-full px-5 pt-28 pb-10">
    <Nav />
    {children}
  </div>
);
