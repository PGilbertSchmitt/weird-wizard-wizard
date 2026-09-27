import { ModalProviderContext } from '@/contexts/modal-context';
import { last } from 'ramda';
import { useContext } from 'react';

export const useModal = () => {
  const { pushModal, popModal, modalStack } = useContext(ModalProviderContext);

  const pushError = (error: React.ReactNode) => pushModal({
    type: 'Error',
    error,
  });

  const popNonErrorModal = () => {
    const lastModal = last(modalStack);
    if (lastModal && lastModal.type !== 'Error') {
      popModal();
    }
  };
  
  return {
    modalStack,
    pushModal,
    pushError,
    popModal,
    popNonErrorModal,
  };
};
