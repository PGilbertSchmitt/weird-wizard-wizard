import { err, ok, Result } from 'neverthrow';
import zod, { ZodError, ZodObject } from 'zod';
import { parse as parseCSVRaw } from 'csv-parse/browser/esm/sync';
import { CsvRawParseResult } from '.';

export const pipeSeparatedValues = (column: string): string[] =>
  column === '' ? [] : column.split('|').map((s) => s.trim());

export const Validations = {
  STRING: zod.string().nonempty(),
  SIZE_REQUIRED: zod.literal(
    ['sm', 'md', 'lg'],
    "Expected size to be one of 'sm', 'md', or 'lg'.",
  ),
  SIZE: zod.literal(
    ['', 'sm', 'md', 'lg'],
    "Expected size to be one of 'sm', 'md', or 'lg'.",
  ),
  POS_NUM: zod.coerce
    .number()
    .int()
    .positive('Expected a positive number, not zero or blank'),
  POS_OR_ZERO: zod.coerce
    .number()
    .int()
    .nonnegative(
      'Expected a positive number, zero, or blank, not a negative number.',
    ),
  PIPE_DELIM_ARRAY: zod.string().transform(pipeSeparatedValues),
  BOOL: zod.literal(['', 'TRUE', 'FALSE']).transform((v) => v === 'TRUE'),
};

export const formatZodError = (error: ZodError, rowNumber: number) => {
  return error.issues.map(
    (subError) =>
      `[Row ${rowNumber}][Column ${String(subError.path[0])}] ${subError.message}`,
  );
};

export const parseRow =
  <T extends {}>(validator: ZodObject<T>) =>
  (
    record: Record<string, string>,
    rowNumber: number,
  ): Result<zod.output<ZodObject<T>>, string[]> => {
    if (!record || typeof record !== 'object') {
      return err([`[Row ${rowNumber}] Row was not parsable`]);
    }

    try {
      const result = validator.parse(record);
      return ok(result);
    } catch (error) {
      if (error instanceof ZodError) {
        return err(formatZodError(error, rowNumber));
      } else {
        return err([
          `[Row ${rowNumber}] Unexpected error while parsing row: ${error}`,
        ]);
      }
    }
  };

export const parseCSV = (data: Uint8Array): CsvRawParseResult =>
  parseCSVRaw(data, {
    columns: true,
  });
