import { cn } from '@/lib/utils';
import { createRef, DragEvent, useEffect, useState } from 'react';
import { listen, Event as TauriEvent } from '@tauri-apps/api/event';
import { Button } from '@/components/ui/button';
import { Trash } from 'lucide-react';
import {
  ExtractError,
  extractFromBuffer,
  extractFromFilename,
  ExtractResult,
} from '../../../lib/import-data/unzip';
import { ImportData } from '@/lib/import-data';
import { ExtractErrorAlert } from './extract-error';
import { fromPairs, toPairs } from 'ramda';
import { importProcess } from '@/lib/import-data/import-process';

export const Dropzone = () => {
  const fileRef = createRef<HTMLInputElement>();
  const dropRef = createRef<HTMLDivElement>();
  const [overDropzone, setOverDropzone] = useState(false);

  const [fileSource, setFileSource] = useState<string | null>(null);
  const [csvData, setCsvData] = useState<ImportData | null>(null);
  const [alertInfo, setAlertInfo] = useState<ExtractError | null>(null);

  const onDragEnter = (e: DragEvent) => {
    e.stopPropagation();
    e.preventDefault();
    setOverDropzone(true);
  };

  const onDragLeave = (e: DragEvent) => {
    e.stopPropagation();
    e.preventDefault();
    setOverDropzone(false);
  };

  const extractionCallback = (filename: string) => (result: ExtractResult) => {
    if (result.isOk()) {
      setFileSource(filename);
      const data = result.value;
      console.log(data);
      setCsvData(data);
      const warnings = toPairs(data).reduce(
        (acc, [file, rowResults]): [string, string][] => {
          const badRows = rowResults.filter((r) => r.isErr()).length;
          if (badRows > 0) {
            acc.push([file, `${badRows} bad rows out of ${rowResults.length}`]);
          }
          return acc;
        },
        [],
      );
      if (warnings.length > 0) {
        setAlertInfo({
          title: 'Row warnings',
          body: fromPairs(warnings),
        });
      }
      // console.log('Total records:', importProcess(data, () => {}));
    } else {
      setAlertInfo(result.error);
      setCsvData(null);
      setFileSource(null);
    }
  };

  useEffect(() => {
    if (overDropzone) {
      const unlistenPromise = listen(
        'tauri://drag-drop',
        (event: TauriEvent<{ paths: string[] }>) => {
          const filepath = event.payload.paths[0];
          extractFromFilename(filepath).then(extractionCallback(filepath));
        },
      );

      return () => {
        unlistenPromise.then((unlisten) => unlisten());
      };
    }
  }, [overDropzone]);

  return (
    <>
      <div
        ref={dropRef}
        className={cn(
          `w-full my-5 border-foreground border-2 border-dashed rounded-base
          flex items-center justify-center p-20 cursor-pointer bg-background`,
          overDropzone && 'brightness-90',
        )}
        onClick={() => {
          fileRef.current?.click();
        }}
        onDragEnter={onDragEnter}
        onDragLeave={onDragLeave}
      >
        <input
          type="file"
          hidden
          ref={fileRef}
          accept="zip"
          multiple={false}
          onChange={async (e) => {
            const file = e.target.files?.item(0);
            if (file) {
              extractFromBuffer(
                new Uint8Array(await file.arrayBuffer()),
                file.name,
              ).then(extractionCallback(file.name));
            }
          }}
        />

        {fileSource
          ? pretty(fileSource)
          : 'Drop file or click here to upload a data zip file.'}
      </div>

      {csvData && (
        <div className={cn('w-full flex')}>
          <Button
            className={cn('w-10 mr-2')}
            onClick={() => {
              setCsvData(null);
              setFileSource(null);
            }}
          >
            <Trash size="20px" strokeWidth="1.2px" />
          </Button>
          <Button
            className={cn('w-full')}
            onClick={() => {
              console.log('doot!');
              importProcess(csvData, () => {});
            }}
          >
            Import
          </Button>
        </div>
      )}

      {alertInfo && (
        <ExtractErrorAlert error={alertInfo} onOk={() => setAlertInfo(null)} />
      )}
    </>
  );
};

// Useful for prettier rendering of the path by adding a zero-width
// space after every forward slash.
// Thie probably won't work correctly on Windows, so will need to add
// a different source pattern).
const pretty = (str: string) => {
  return str.replace(/\//g, '/\u200B');
};
