import React, { useEffect, useState } from 'react';
import {
  ThemeColor,
  ThemeLight,
  ThemeProviderContext,
  ThemeProviderState,
  ThemeString,
} from '@/contexts/theme-context';

const THEME_STORAGE_KEY = 'www-theme';
const DEFAULT_THEME = 'light-red';

const ALL_THEMES: Array<ThemeLight | ThemeColor> = [
  'light',
  'dark',
  'red',
  'cyan',
  'amber',
];

interface ThemeProviderProps {
  children: React.ReactNode;
}

export const ThemeProvider = ({ children }: ThemeProviderProps) => {
  const [theme, setTheme] = useState(
    () =>
      (localStorage.getItem(THEME_STORAGE_KEY) as ThemeString) || DEFAULT_THEME,
  );

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove(...ALL_THEMES);

    const [light, color] = theme.split('-') as [ThemeLight, ThemeColor];

    // The `.light` class isn't explicitly needed (just the absence of `.dark`)
    // but it doesn't hurt to add it to simplify the logic.
    root.classList.add(light, color);
  }, [theme]);

  const state: ThemeProviderState = {
    theme,
    setTheme: (theme: ThemeString) => {
      localStorage.setItem(THEME_STORAGE_KEY, theme);
      setTheme(theme);
    },
  };

  return (
    <ThemeProviderContext.Provider value={state}>
      {children}
    </ThemeProviderContext.Provider>
  );
};
