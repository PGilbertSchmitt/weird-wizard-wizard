import { ModalData } from '@/components/modals/type';
import { createContext } from 'react';

export interface ModalContextState {
  modalStack: Array<ModalData>;
  pushModal: (modal: ModalData) => void;
  popModal: () => void;
}

const initialState: ModalContextState = {
  modalStack: [],
  pushModal: (_) => null,
  popModal: () => null,
};

export const ModalProviderContext = createContext(initialState);
