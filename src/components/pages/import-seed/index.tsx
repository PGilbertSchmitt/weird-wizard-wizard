import { cn } from '@/lib/utils';
import { Paragraph } from '@/components/ui/paragraph';
import { ExtLink } from '@/components/ui/external-link';
import { Dropzone } from './dropzone';
import { useEffect, useReducer, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Event, listen } from '@tauri-apps/api/event';
import {
  cancelAction,
  DEFAULT_IMPORT_STATE,
  ImportAction,
  ImportActionTypes,
  ImportData,
  ImportStatuses,
  receiveDoneAction,
  receiveReadyAction,
  sendFileAction,
  sendStartAction,
} from './import-state';
import { IpcResult } from '@/types/ipc-result';
import { unwrapIpcResult } from '@/api/request';
import { Button } from '@/components/ui/neo/button';
import { ImportEvent } from '@/types/import';

export const ImportSeed = () => {
  const [importState, dispatch] = useReducer(
    (prev: ImportData, { type, data }: ImportAction) => {
      switch (type) {
        case ImportActionTypes.CANCEL:
          if ('unlistener' in prev) {
            prev.unlistener.then((unlisten) => unlisten());
          }
          return DEFAULT_IMPORT_STATE;

        case ImportActionTypes.SEND_FILE:
          if (prev.status === ImportStatuses.IDLE) {
            const unlistenPromise = listen(
              'IMPORT',
              (event: Event<IpcResult<ImportEvent>>) => {
                try {
                  const payload = unwrapIpcResult(event.payload);
                  switch (payload.type) {
                    case 'Ready': {
                      console.log(
                        'Records waiting to be seeded:',
                        payload.data,
                      );
                      dispatch(receiveReadyAction());
                      break;
                    }
                    case 'Progress': {
                      console.log(
                        `Progress update: ${payload.data[0]} out of ${payload.data[1]}`,
                      );
                      break;
                    }
                    case 'Done': {
                      dispatch(receiveDoneAction());
                      break;
                    }
                    default: {
                      console.log('Unknown:', payload);
                    }
                  }
                } catch (err) {
                  console.error(err);
                  dispatch(cancelAction());
                }
              },
            );

            unlistenPromise.then(() => {
              console.log(`Invoking "init_seed" signal with ${data}`);
              invoke('init_seed', { filepath: data }).catch((err) => {
                console.error(err);
                dispatch(cancelAction());
              });
            });

            return {
              status: ImportStatuses.UNWRAPPING,
              filename: data,
              unlistener: unlistenPromise,
            };
          } else {
            return prev;
          }

        case ImportActionTypes.RECEIVE_READY:
          return prev.status === ImportStatuses.UNWRAPPING
            ? { ...prev, status: ImportStatuses.READY }
            : prev;

        case ImportActionTypes.SEND_START:
          if (prev.status === ImportStatuses.READY) {
            invoke('run_seed').catch((err) => {
              console.error(err);
              dispatch(cancelAction());
            });
            return { ...prev, status: ImportStatuses.IMPORTING };
          } else {
            return prev;
          }

        case ImportActionTypes.RECEIVE_PROGRESS:
          return prev.status === ImportStatuses.IMPORTING
            ? { ...prev /* update progress */ }
            : prev;

        case ImportActionTypes.RECEIVE_DONE:
          if (prev.status === ImportStatuses.IMPORTING) {
            // The reducer is synchronous, so it's theoretically possible for an event to come in
            // before the unlistener is resolved and called. However, this is unlikely to happen
            // after the `DONE` signal is returned, and even if it did, the reducer is setup such
            // that the sendFile action is the only one that can move out of the IDLE state.
            prev.unlistener.then((unlisten) => unlisten());
            alert('yey!');
            return DEFAULT_IMPORT_STATE;
          } else {
            return prev;
          }

        default:
          return prev;
      }
    },
    DEFAULT_IMPORT_STATE,
  );

  const [id, setId] = useState('');

  useEffect(() => {
    return () => {
      if ('unlistener' in importState) {
        importState.unlistener.then((unlisten) => unlisten);
      }
    };
  }, [importState]);

  return (
    <>
      <h1>Import Static Data ({importState.status})</h1>
      <div className={cn('flex flex-col items-center max-w-100')}>
        <Paragraph>
          Static data refers to <i>Shadow of the Weird Wizard</i>
          -specific information that is needed to create a character. This
          includes things like ancestries, paths, traditions, and spells. This
          tool does not ship with this data because it's not my data to
          distribute (it belongs to Robert J. Schwalb and{' '}
          <ExtLink href="https://schwalbentertainment.com/">
            Schwalb Entertainment
          </ExtLink>
          ).
        </Paragraph>
        <Paragraph>
          If you have zip file of the seed data, you can drop it off here. If
          you don't have one yet, you can create one if you have the{' '}
          <ExtLink href="https://schwalbentertainment.com/shadow-of-the-weird-wizard/">
            SofWW rule book
          </ExtLink>{' '}
          and a bunch of time on your hands.
        </Paragraph>

        <Dropzone
          fileSource={importState.filename}
          disabled={
            importState.status === ImportStatuses.UNWRAPPING ||
            importState.status === ImportStatuses.IMPORTING
          }
          onFilePath={async (filepath) => {
            console.log('Dispatch send-file');
            dispatch(sendFileAction(filepath));
          }}
        />

        {importState.status !== ImportStatuses.IDLE && (
          <Button onClick={() => dispatch(cancelAction())}>Cancel</Button>
        )}

        {importState.status === ImportStatuses.READY && (
          <Button onClick={() => dispatch(sendStartAction())}>Do it!</Button>
        )}

        <input value={id} onChange={(e) => setId(e.target.value)} />

        <Button
          disabled={Number.isNaN(parseInt(id))}
          onClick={() => {
            invoke('get_magic_talent', { id: parseInt(id) }).then((data) => {
              console.log('une data', data);
            });
          }}
        >
          CLOCK
        </Button>
      </div>
    </>
  );
};
