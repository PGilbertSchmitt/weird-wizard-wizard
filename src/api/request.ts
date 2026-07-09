import { IpcResult } from '@/types/ipc-result';

// Results sent via Tauri's commands are unwrapped automatically, but not for
// Results sent via the event emitter, so this function performs that step.
export const unwrapIpcResult = <T = void>(res: IpcResult<T>) => {
  if ('Ok' in res) {
    return res.Ok.data;
  } else {
    throw new Error(res.Err.message);
  }
};
