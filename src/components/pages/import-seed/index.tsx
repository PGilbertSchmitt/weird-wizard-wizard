import { Paragraph } from '@/components/ui/paragraph';
import { cn } from '@/lib/utils';
import { Dropzone } from './dropzone';
import { Button } from '@/components/ui/button';
import { Trash } from 'lucide-react';
import { importProcess } from '@/lib/import-data/import-process';
import { ExtractErrorAlert } from './extract-error';
import { useState } from 'react';
import {
  ExtractError,
  extractFromBuffer,
  extractFromFilename,
  ExtractResult,
} from '@/lib/import-data/unzip';
import { ImportData } from '@/lib/import-data';
import { fromPairs, toPairs } from 'ramda';
import { ImportLoadProgress } from './import-status';

export const ImportSeed = () => {
  const [fileSource, setFileSource] = useState<string | null>(null);
  const [csvData, setCsvData] = useState<ImportData | null>(null);
  const [alertInfo, setAlertInfo] = useState<ExtractError | null>(null);
  const [importing, setImporting] = useState(false);
  const [currentItem, setCurrentItem] = useState('');
  const [currentPercent, setCurrentPercent] = useState(0);
  const [done, setDone] = useState(false);

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
    } else {
      setAlertInfo(result.error);
      setCsvData(null);
      setFileSource(null);
    }
  };

  return (
    <>
      <h1>Import Static Data</h1>
      <div className={cn('flex flex-col items-center max w-100')}>
        <Paragraph>
          Static data refers to <i>Shadow of the Weird Wizard</i>-specific
          information that is needed to create a character. This includes things
          like ancestries, paths, traditions, and spells. This tool does not
          ship with this data because it's not my data to distribute (it belongs
          to Robert J. Schwalb and{' '}
          <a className="default" href="https://schwalbentertainment.com/">
            Schwalb Entertainment
          </a>
          ).
        </Paragraph>
        <Paragraph>
          If you have zip file of the seed data, you can drop it off here. If
          you don't have one yet, you can create one if you have the{' '}
          <a
            className="default"
            href="https://schwalbentertainment.com/shadow-of-the-weird-wizard/"
          >
            SofWW rule book
          </a>{' '}
          and a bunch of time on your hands.
        </Paragraph>

        <Dropzone
          fileSource={fileSource}
          disabled={importing}
          onFileHandle={async (file) => {
            extractFromBuffer(
              new Uint8Array(await file.arrayBuffer()),
              file.name,
            ).then(extractionCallback(file.name));
          }}
          onFilePath={async (filepath) => {
            extractFromFilename(filepath).then(extractionCallback(filepath));
          }}
        />

        {csvData && (
          <div className={cn('w-full flex')}>
            <Button
              className={cn('w-10 mr-2')}
              disabled={importing}
              onClick={() => {
                setCsvData(null);
                setFileSource(null);
              }}
            >
              <Trash size="20px" strokeWidth="1.2px" />
            </Button>
            <Button
              className={cn('w-full')}
              disabled={importing}
              onClick={() => {
                setImporting(true);
                importProcess(csvData, (item, percent) => {
                  setCurrentItem(item);
                  setCurrentPercent(percent);
                }).then(() => {
                  setDone(true);
                  setFileSource(null);
                  setCsvData(null);
                  setCurrentItem('');
                  setCurrentPercent(0);
                });
              }}
            >
              Import
            </Button>
          </div>
        )}

        {alertInfo && (
          <ExtractErrorAlert
            error={alertInfo}
            onOk={() => setAlertInfo(null)}
          />
        )}

        {importing && (
          <ImportLoadProgress
            currentItem={currentItem}
            currentPercent={currentPercent}
            done={done}
            onOk={() => {
              setDone(false);
              setImporting(false);
            }}
          />
        )}
      </div>
    </>
  );
};
