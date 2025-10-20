import { createContext } from "react";

export type ThemeLight = 'light' | 'dark';
export type ThemeColor = 'red' | 'cyan';
export type ThemeString = `${ThemeLight}-${ThemeColor}`;

export interface ThemeProviderState {
    theme: ThemeString;
    setTheme: (theme: ThemeString) => void;
}

const initialState: ThemeProviderState = {
    theme: 'light-red',
    setTheme: () => null,
};

export const ThemeProviderContext =
    createContext<ThemeProviderState>(initialState);
