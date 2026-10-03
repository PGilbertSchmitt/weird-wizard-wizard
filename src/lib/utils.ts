import { clsx, type ClassValue } from 'clsx';
import { init, last } from 'ramda';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export type ValueOf<T> = T[keyof T];

/** Converts a number into a Nth string, eg 1st, 2nd, 3rd, 4th, etc */
export const nth = (idx: number): string => {
  const str = (idx + 1).toString();
  switch (last(str)) {
    case '1':
      return `${str}st`;
    case '2':
      return `${str}nd`;
    case '3':
      return `${str}rd`;
    default:
      return `${str}th`;
  }
};

export const orSeparatedStr = (sources: string[]): string => {
  switch (sources.length) {
    case 0:
      return '';
    case 1:
      return sources[0];
    case 2:
      return `${sources[0]} or ${sources[1]}`;
    default:
      return `${init(sources).join(', ')}, or ${last(sources)}`;
  }
};
