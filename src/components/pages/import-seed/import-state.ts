import { ImportSummary, ProgressPayload } from '@/types/import';
import { UnlistenFn } from '@tauri-apps/api/event';

export const ImportStatuses = {
  IDLE: 'IDLE',
  UNWRAPPING: 'UNWRAPPING',
  READY: 'READY',
  IMPORTING: 'IMPORTING',
  ERROR: 'ERROR',
} as const;

interface IdleState {
  status: typeof ImportStatuses.IDLE;
  filename: null;
}

interface UnwrappingState {
  status: typeof ImportStatuses.UNWRAPPING;
  filename: string;
  unlistener: Promise<UnlistenFn>;
}

interface ReadyState extends Omit<UnwrappingState, 'status'> {
  status: typeof ImportStatuses.READY;
  summary: ImportSummary;
  total: number;
}

interface ImportingState extends Omit<ReadyState, 'status'> {
  status: typeof ImportStatuses.IMPORTING;
  current: number;
  total: number;
}

interface ErrorState {
  status: typeof ImportStatuses.ERROR;
  filename: null;
  error: string;
}

export const DEFAULT_IMPORT_STATE: IdleState = {
  status: ImportStatuses.IDLE,
  filename: null,
};

export type ImportData =
  IdleState | UnwrappingState | ReadyState | ImportingState | ErrorState;

export const ImportActionTypes = {
  SEND_FILE: 'SEND_FILE',
  RECEIVE_READY: 'RECEIVE_READY',
  SEND_START: 'SEND_START',
  RECEIVE_PROGRESS: 'RECEIVE_PROGRESS',
  RECEIVE_DONE: 'RECEIVE_DONE',
  ERROR: 'ERROR',
  CANCEL: 'CANCEL',
} as const;

export const sendFileAction = (filepath: string) => ({
  type: ImportActionTypes.SEND_FILE,
  data: filepath,
});

export const receiveReadyAction = (data: ImportSummary) => ({
  type: ImportActionTypes.RECEIVE_READY,
  data,
});

export const sendStartAction = () => ({
  type: ImportActionTypes.SEND_START,
  data: null,
});

export const receiveProgressAction = (data: ProgressPayload) => ({
  type: ImportActionTypes.RECEIVE_PROGRESS,
  data,
});

export const receiveDoneAction = () => ({
  type: ImportActionTypes.RECEIVE_DONE,
  data: null,
});

export const cancelAction = () => ({
  type: ImportActionTypes.CANCEL,
  data: null,
});

export const errorAction = (err: string) => ({
  type: ImportActionTypes.ERROR,
  data: err,
});

export type ImportAction =
  | ReturnType<typeof sendFileAction>
  | ReturnType<typeof receiveReadyAction>
  | ReturnType<typeof sendStartAction>
  | ReturnType<typeof receiveProgressAction>
  | ReturnType<typeof receiveDoneAction>
  | ReturnType<typeof cancelAction>
  | ReturnType<typeof errorAction>;
