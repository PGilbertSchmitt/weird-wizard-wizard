import {
  ModalContextState,
  ModalProviderContext,
} from '@/contexts/modal-context';
import { init } from 'ramda';
import { useState } from 'react';
import { ModalData } from '../modals/type';

export const ModalProvider = ({ children }: { children: React.ReactNode }) => {
  const [modalStack, setModalStack] = useState<ModalData[]>([]);

  const state: ModalContextState = {
    modalStack,
    pushModal: (modal) => setModalStack([...modalStack, modal]),
    popModal: () => setModalStack(init(modalStack)),
  };

  return <ModalProviderContext value={state}>{children}</ModalProviderContext>;
};
