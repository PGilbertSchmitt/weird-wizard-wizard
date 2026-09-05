import { ThemeProviderContext } from '@/contexts/theme-context';
import {
  ThemeColor,
  ThemeContextState,
  ThemeLight,
} from '@/contexts/theme-context';
import { useEffect, useState } from 'react';

const THEME_STORAGE_KEY = 'www-theme';

interface ThemeData {
  light: ThemeLight;
  color: ThemeColor;
}
const DEFAULT_THEME: ThemeData = { light: 'light', color: 'red' };
const THEME_LIGHTS: ThemeLight[] = ['light', 'dark'];
const THEME_COLORS: ThemeColor[] = ['red', 'cyan', 'amber'];
const ALL_THEMES: Array<ThemeLight | ThemeColor> = [
  ...THEME_LIGHTS,
  ...THEME_COLORS,
];

const isLight = (s: string): s is ThemeLight => {
  return (THEME_LIGHTS as string[]).includes(s);
};

const isColor = (s: string): s is ThemeColor => {
  return (THEME_COLORS as string[]).includes(s);
};

const parseThemeString = (theme: string | null): ThemeData => {
  if (theme !== null) {
    const [light, color] = theme.split(':').map((s) => s.trim());

    if (isLight(light) && isColor(color)) {
      return { light, color };
    }

    console.warn(
      `Invalid theme '${theme}' set in localstorage, defaulting to light red`,
    );
  }

  return DEFAULT_THEME;
};

export const ThemeProvider = ({ children }: { children: React.ReactNode }) => {
  const [theme, setTheme] = useState(() =>
    parseThemeString(localStorage.getItem(THEME_STORAGE_KEY)),
  );

  const state: ThemeContextState = {
    ...theme,
    setLight: (light) => {
      localStorage.setItem(THEME_STORAGE_KEY, `${light}-${theme.color}`);
      setTheme({ light, color: theme.color });
    },
    setColor: (color) => {
      localStorage.setItem(THEME_STORAGE_KEY, `${theme.light}-${color}`);
      setTheme({ light: theme.light, color });
    },
  };

  useEffect(() => {
    const root = document.documentElement;
    root.classList.remove(...ALL_THEMES);
    root.classList.add(theme.light, theme.color);
  }, [theme.color, theme.light]);

  return <ThemeProviderContext value={state}>{children}</ThemeProviderContext>;
};
