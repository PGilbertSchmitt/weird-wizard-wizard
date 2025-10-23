import { exists, readFile } from '@tauri-apps/plugin-fs';
import JSZip, { loadAsync as loadZipAsync } from 'jszip';
import { ok, err, Result } from 'neverthrow';
import { parseAncestryCSV } from '@/lib/import-data/ancestry-import';
import {
  BufferFiles,
  CsvResults,
  CsvFiles,
  ImportData,
} from '@/lib/import-data';
import { fromPairs, keys, last, toPairs } from 'ramda';
import { parseLevelCSV } from '@/lib/import-data/level-import';
import { parseCSV } from '@/lib/import-data/import-utils';
import { parseNonNovicePathCSV, parseNovicePathCSV } from './path-import';
import { parseMagicOptionCSV, parseMagicSpellCSV, parseMagicTableCSV, parseMagicTalentCSV, parseMagicTraditionCSV } from './magic-import';

export type ExtractError = {
  title: string;
} & (
  {
    body: Record<string, string>;
  } | {
    message: string | string[];
  }
);

export type ExtractResult = Result<ImportData, ExtractError>;

export const extractFromFilename = async (filepath: string): Promise<ExtractResult> => {
  if (!(await exists(filepath))) {
    return err({
      title: 'File Error',
      message: `'${filepath}' could not be found.`,
    });
  }

  let buffer: Uint8Array<ArrayBuffer>;
  try {
    buffer = await readFile(filepath);
  } catch (error) {
    console.log(error);
    return err({
      title: 'File Error',
      message: `Failed to read file '${filepath}'`,
    });
  }

  return extractFromBuffer(buffer, last(filepath.split('/')));
};

export const extractFromBuffer = async (buffer: Uint8Array, filename: string = ''): Promise<ExtractResult> => {
  let unzippedFile: JSZip;
  try {
    unzippedFile = await loadZipAsync(buffer);
  } catch (error) {
    console.error(error);
    return err({
      title: 'Zip Error',
      message: `Could not unzip '${filename}', are you sure it was a zip archive?`,
    });
  }

  try {
    return await extractCsvFiles(unzippedFile);
  } catch (error) {
    return err({
      title: 'Unexpected Error',
      message: JSON.stringify(error),
    });
  }
}

const extractCsvFiles = async (unzippedFile: JSZip): Promise<ExtractResult> => {
  const files = unzippedFile.files;
  const missingFiles: string[] = [];
  for (const filename of keys(CsvFiles)) {
    if (!(filename in files)) {
      missingFiles.push(filename);
    }
  }
  if (missingFiles.length > 0) {
    return err({
      title: 'These files are missing in the ZIP',
      message: missingFiles,
    });
  }

  const parseErrors: Record<string, string> = {};
  const csvResults = fromPairs(
    await Promise.all(
      toPairs(BufferFiles).map(
        async ([key, filename]) => {
          try {
            return [key, parseCSV(await files[filename].async('uint8array'))];
          } catch (error) {
            const errorString =
              typeof error === 'string'
              ? error
              : error !== null && typeof error === 'object' && 'toString' in error
              ? error.toString()
              : JSON.stringify(error);
            parseErrors[filename] = errorString;

            // Just for type checker
            return ['', []];
          }
        },
      ),
    ),
  ) as CsvResults; // This typecast is valid as long as parseErrors is empty

  if (keys(parseErrors).length > 0) {
    return err({
      title: 'CSV Parse Error',
      body: parseErrors,
    });
  }

  return ok({
    ancestries: parseAncestryCSV(csvResults.ancestriesBuffer),
    noviceLevels: parseLevelCSV(csvResults.noviceLevelsBuffer),
    expertLevels: parseLevelCSV(csvResults.expertLevelsBuffer),
    masterLevels: parseLevelCSV(csvResults.masterLevelsBuffer),
    novicePaths: parseNovicePathCSV(csvResults.novicePathsBuffer),
    expertPaths: parseNonNovicePathCSV(csvResults.expertPathsBuffer),
    masterPaths: parseNonNovicePathCSV(csvResults.masterPathsBuffer),
    magicTraditions: parseMagicTraditionCSV(csvResults.magicTraditionsBuffer),
    magicTalents: parseMagicTalentCSV(csvResults.magicTalentsBuffer),
    magicSpells: parseMagicSpellCSV(csvResults.magicSpellsBuffer),
    magicTables: parseMagicTableCSV(csvResults.magicTablesBuffer),
    magicOptions: parseMagicOptionCSV(csvResults.magicOptionsBuffer),
  });
};
