import {
  ModalContextState,
  ModalProviderContext,
} from '@/contexts/modal-context';
import { init } from 'ramda';
import { useEffect, useState } from 'react';
import { ModalData } from '../modals/type';

export const ModalProvider = ({ children }: { children: React.ReactNode }) => {
  const [modalStack, setModalStack] = useState<ModalData[]>([]);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && modalStack.length > 0 && modalStack[0].type !== 'Error') {
        setModalStack(init(modalStack));
      }
    }
    document.addEventListener('keydown', handler);

    return () => document.removeEventListener('keydown', handler);
  }, [modalStack, setModalStack]);

  const state: ModalContextState = {
    modalStack,
    pushModal: (modal) => setModalStack([...modalStack, modal]),
    popModal: () => setModalStack(init(modalStack)),
  };

  return <ModalProviderContext value={state}>{children}</ModalProviderContext>;
};
