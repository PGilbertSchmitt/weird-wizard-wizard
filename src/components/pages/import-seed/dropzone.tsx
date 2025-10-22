import { cn } from "@/lib/utils";
import { createRef, DragEvent, useEffect, useState } from "react";
import { listen, Event as TauriEvent } from '@tauri-apps/api/event';
import { Button } from "@/components/ui/button";
import { Trash } from "lucide-react";
import { unzip } from "./unzip";

export const Dropzone = () => {
  const fileRef = createRef<HTMLInputElement>();
  const dropRef = createRef<HTMLDivElement>();
  const [overDropzone, setOverDropzone] = useState(false);

  const [fileSource, setFileSource] = useState<string | null>(null);
  const [csvData, setCsvData] = useState<unknown>(null);

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

  useEffect(() => {
    if (overDropzone) {
      const unlistenPromise = listen('tauri://drag-drop', (event: TauriEvent<{ paths: string[] }>) => {
        console.log(overDropzone, event);
        setFileSource(event.payload.paths[0]);
      });

      return () => {
        unlistenPromise.then(unlisten => unlisten());
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
          overDropzone && 'brightness-90'
        )}
        onClick={() => {
          fileRef.current?.click();
        }}
        onDragEnter={onDragEnter}
        onDragLeave={onDragLeave}
      >
        <input type='file' hidden ref={fileRef} />
        
        {fileSource ? (
          pretty(fileSource)
        ) : (
          "Drop file or click here to upload a data zip file."
        )}
      </div>

      {fileSource && (
        <div className={cn('w-full flex')}>
          <Button
            className={cn('w-10 mr-2')}
            onClick={() => {
              setCsvData(null);
              setFileSource(null);
            }}
          >
            <Trash size='20px' strokeWidth="1.2px" />
          </Button>
          <Button
            className={cn('w-full')}
            onClick={() => {
              unzip(fileSource).then(data => {
                setCsvData(data);
                console.log(data);
              });
            }}  
          >
            Import
          </Button>
        </div>
      )}

      {csvData && (
        <div>
          We got it!
        </div>
      )}
    </>
  );
};

// Useful for prettier rendering of the path by adding a zero-width
// space after every forward slash.
// Thie probably won't work correctly on Windows, so will need to add
// a different source pattern).
const pretty = (str: string) => {
  return str.replace(/\//g, "/\u200B");
};
