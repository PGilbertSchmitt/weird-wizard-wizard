import { createContext } from 'react';

export type ThemeLight = 'light' | 'dark';
export type ThemeColor = 'red' | 'cyan' | 'amber';

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

export const ThemeProviderContext =
  createContext<ThemeContextState>(initialState);
