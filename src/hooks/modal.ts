import { ModalProviderContext } from '@/contexts/modal-context';
import { useContext } from 'react';

export const useModal = () => {
  const { pushModal, popModal } = useContext(ModalProviderContext);
  return { pushModal, popModal };
};
