export interface IpcResponse<T> {
    Ok: {
        data: T;
    };
}

export interface IpcFailure {
    Err: {
        message: string;
    };
}

export type IpcResult<T> = IpcResponse<T> | IpcFailure;
