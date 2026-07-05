import { IpcResult } from '@/types/ipc-result';
import { invoke, InvokeArgs } from '@tauri-apps/api/core';

export const unwrapIpcResult = <T = void>(res: IpcResult<T>) => {
  if ('Ok' in res) {
    return res.Ok.data;
  } else {
    throw new Error(res.Err.message);
  }
};

/**
 * Wrapper around Tauri's invoke function to unwrap my IpcResult type.
 */
export const request = async <T = void, U = InvokeArgs>(
  command: string,
  params?: U,
) => {
  const res: IpcResult<T> = await invoke(command, params || {});
  return unwrapIpcResult(res);
};
