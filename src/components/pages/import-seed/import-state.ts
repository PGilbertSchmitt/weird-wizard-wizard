import { UnlistenFn } from '@tauri-apps/api/event';

export const ImportStatuses = {
    IDLE: 'IDLE',
    UNWRAPPING: 'UNWRAPPING',
    READY: 'READY',
    IMPORTING: 'IMPORTING',
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
    // should also contain info about resources
}

interface ImportingState extends Omit<ReadyState, 'status'> {
    status: typeof ImportStatuses.IMPORTING;
    // should also contain info about progress
}

export const DEFAULT_IMPORT_STATE: IdleState = {
    status: ImportStatuses.IDLE,
    filename: null,
};

export type ImportData =
    IdleState | UnwrappingState | ReadyState | ImportingState;

export const ImportActionTypes = {
    SEND_FILE: 'SEND_FILE',
    RECEIVE_READY: 'RECEIVE_READY',
    SEND_START: 'SEND_START',
    RECEIVE_PROGRESS: 'RECEIVE_PROGRESS',
    RECEIVE_DONE: 'RECEIVE_DONE',
    CANCEL: 'CANCEL',
} as const;

export const sendFileAction = (filepath: string) => ({
    type: ImportActionTypes.SEND_FILE,
    data: filepath,
});

export const receiveReadyAction = () => ({
    type: ImportActionTypes.RECEIVE_READY,
    data: true, // Summary of records
});

export const sendStartAction = () => ({
    type: ImportActionTypes.SEND_START,
    data: null,
});

export const receiveProgressAction = () => ({
    type: ImportActionTypes.RECEIVE_PROGRESS,
    data: null,
});

export const receiveDoneAction = () => ({
    type: ImportActionTypes.RECEIVE_DONE,
    data: null,
});

export const cancelAction = () => ({
    type: ImportActionTypes.CANCEL,
    data: null,
});

export type ImportAction =
    | ReturnType<typeof sendFileAction>
    | ReturnType<typeof receiveReadyAction>
    | ReturnType<typeof sendStartAction>
    | ReturnType<typeof receiveProgressAction>
    | ReturnType<typeof receiveDoneAction>
    | ReturnType<typeof cancelAction>;
