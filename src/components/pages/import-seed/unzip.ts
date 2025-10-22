import { exists, readFile } from '@tauri-apps/plugin-fs';
import JSZip, { loadAsync as loadZipAsync } from 'jszip';
import { ok, err, Result } from 'neverthrow';
import { parseAncestryCSV } from '@/lib/import-data/ancestry';
import { BufferFiles, CsvBuffers, CsvFiles, ImportData } from '@/lib/import-data';
import { fromPairs, keys, toPairs } from 'ramda';
import { parseLevelCSV } from '@/lib/import-data/levels';


interface ZipData {};
type ZipResult = Result<ZipData, string>;

export const unzip = async (filepath: string): Promise<ZipResult> => {
  if (!(await exists(filepath))) {
    return err(`File '${filepath}' could not be found.`);
  }

  let buffer: Uint8Array<ArrayBuffer>;
  try {
    buffer = await readFile(filepath);
  } catch (error) {
    console.log(error);
    return err(`Failed to read file '${filepath}'`);
  }

  let unzippedFile: JSZip;
  try {
    unzippedFile = await loadZipAsync(buffer);
  } catch (error) {
    console.error(error);
    return err(`Could not process '${filepath}', are you sure it was a zip archive?`);
  }

  try {
    return ok(await extractCsvFiles(unzippedFile));
  } catch (error) {
    return err(error as string);
  }
};

const extractCsvFiles = async (unzippedFile: JSZip): Promise<ImportData> => {
  const files = unzippedFile.files;
  const missingFiles: string[] = [];
  for (const filename of keys(CsvFiles)) {
    if (!(filename in files)) {
      missingFiles.push(filename);
    }
  }
  if (missingFiles.length > 0) {
    throw new Error(`Could not find these files: ${missingFiles.join(', ')}`);
  }

  const failedData: string[] = [];
  const csvBuffers = fromPairs(await Promise.all(
    toPairs(BufferFiles).map(async ([key, filename]): Promise<[string, Uint8Array]> => {
      try {
        return [key, await files[filename].async('uint8array')];
      } catch (error) {
        console.log(`While reading ${filename}`, error);
        failedData.push(filename);
        return ['', new Uint8Array()]; // Just for type checker
      }
    }),
  )) as CsvBuffers; // This typecast is valid as long as failedData is empty

  if (failedData.length > 0) {
    throw new Error(`Failed to read these files: ${failedData.join(', ')}`);
  }

  return {
    ancestries: parseAncestryCSV(csvBuffers.ancestriesBuffer),
    noviceLevels: parseLevelCSV(csvBuffers.noviceLevelsBuffer),
    expertLevels: parseLevelCSV(csvBuffers.expertLevelsBuffer),
    masterLevels: parseLevelCSV(csvBuffers.masterLevelsBuffer),
    novicePaths: null,
    expertPaths: null,
    magicOptions: null,
    magicSpells: null,
    magicTables: null,
    magicTalents: null,
    magicTraditions: null,
    masterPaths: null,
  };
};
