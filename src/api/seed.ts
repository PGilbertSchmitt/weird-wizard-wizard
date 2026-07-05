import { useMutation } from '@tanstack/react-query';
import { request } from './request';

export const useInitSeed = (filepath: string) =>
    useMutation({
        mutationFn: () => request('init_seed', { filepath }),
    });
