import { createContext } from 'react';

export type ThemeLight = 'light' | 'dark';
export type ThemeColor = 'red' | 'cyan' | 'amber' | 'green';

export const ALL_COLORS: ThemeColor[] = ['red', 'amber', 'green', 'cyan'];

export interface ThemeContextState {
  light: ThemeLight;
  color: ThemeColor;
  setLight: (light: ThemeLight) => void;
  setColor: (color: ThemeColor) => void;
}

const initialState: ThemeContextState = {
  light: 'light',
  color: 'red',
  setLight: (_) => null,
  setColor: (_) => null,
};

export const ThemeProviderContext = createContext(initialState);
