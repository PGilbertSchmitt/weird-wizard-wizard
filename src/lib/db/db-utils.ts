import { times } from 'ramda';

// To generate sequences such as ($1, $2, $3), ($4, $5, $6), ... for safe params
// The output of this function is safe to interpolate directly into a query string
export const paramPairs = (numParamsPer: number, numSets: number = 1): string => {
  let param = 1;
  return times(() => {
    const paramSet = times(() => `$${param++}`, numParamsPer).join(', ');
    return `(${paramSet})`;
  }, numSets).join(', ');
};
