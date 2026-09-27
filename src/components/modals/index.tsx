import { ModalProviderContext } from '@/contexts/modal-context';
import { init, last } from 'ramda';
import { useContext } from 'react';
import { ModalSwitch } from './switch';

export const Modals = () => {
  const { modalStack } = useContext(ModalProviderContext);

  const activeModal = last(modalStack);
  if (activeModal === undefined) {
    return null;
  }

  const inactiveModals = init(modalStack);

  return (
    <div className="modal-block">
      {inactiveModals.map((inactiveModal, i) => (
        <div key={i} className="modal-box">
          <ModalSwitch modalData={inactiveModal} />
        </div>
      ))}
      <div className="modal-filter" />
      <div className="modal-box">
        <ModalSwitch modalData={activeModal} />
      </div>
    </div>
  );
};
