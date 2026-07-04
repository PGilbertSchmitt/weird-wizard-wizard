import { cn } from '@/lib/utils';
import { Paragraph } from '@/components/ui/paragraph';
import { ExtLink } from '@/components/ui/external-link';
import { Dropzone } from './dropzone';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export const ImportSeed = () => {
  const [fileSource, setFileSource] = useState<string | null>(null);
  const [importing, _setImporting] = useState(false);
  
  return (
    <>
      <h1>Import Static Data</h1>
      <div className={cn("flex flex-col items-center max-w-100")}>
        <Paragraph>
          Static data refers to <i>Shadow of the Weird Wizard</i>-specific
          information that is needed to create a character. This includes things
          like ancestries, paths, traditions, and spells. This tool does not
          ship with this data because it's not my data to distribute (it belongs
          to Robert J. Schwalb and{" "}
          <ExtLink href="https://schwalbentertainment.com/">
            Schwalb Entertainment
          </ExtLink>
          ).
        </Paragraph>
        <Paragraph>
          If you have zip file of the seed data, you can drop it off here. If
          you don't have one yet, you can create one if you have the{" "}
          <ExtLink href="https://schwalbentertainment.com/shadow-of-the-weird-wizard/">
            SofWW rule book
          </ExtLink>{" "}
          and a bunch of time on your hands.
        </Paragraph>

        <Dropzone
          fileSource={fileSource}
          disabled={importing}
          onFilePath={async (filepath) => {
            invoke('init_seed', { filepath })
              .then(() => setFileSource(filepath))
              .catch(err => console.error(err));
          }}
        />
      </div>
    </>
  );
};
