import { cn } from '@/lib/utils';
import { createRef, DragEvent, useEffect, useState } from 'react';
import { listen, Event as TauriEvent } from '@tauri-apps/api/event';

interface DropzoneProps {
  disabled: boolean;
  fileSource: string | null;
  onFileHandle: (file: File) => Promise<void>;
  onFilePath: (filepath: string) => Promise<void>;
}

export const Dropzone = ({
  disabled = false,
  fileSource,
  onFileHandle,
  onFilePath,
}: DropzoneProps) => {
  const fileRef = createRef<HTMLInputElement>();
  const dropRef = createRef<HTMLDivElement>();
  const [overDropzone, setOverDropzone] = useState(false);

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
      const unlistenPromise = listen(
        'tauri://drag-drop',
        (event: TauriEvent<{ paths: string[] }>) => {
          const filepath = event.payload.paths[0];
          onFilePath(filepath);
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
          disabled
            ? 'opacity-50 cursor-default'
            : overDropzone && 'brightness-90',
        )}
        onClick={() => {
          fileRef.current?.click();
        }}
        onDragEnter={onDragEnter}
        onDragLeave={onDragLeave}
      >
        <input
          type="file"
          disabled={disabled}
          hidden
          ref={fileRef}
          accept="zip"
          multiple={false}
          onChange={async (e) => {
            const file = e.target.files?.item(0);
            if (file) {
              onFileHandle(file);
            }
          }}
        />

        {fileSource
          ? pretty(fileSource)
          : 'Drop file or click here to upload a data zip file.'}
      </div>
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
