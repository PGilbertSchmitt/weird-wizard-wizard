export type ImportError = {
  title: string;
  continue?: boolean;
} & (
  | {
      body: Record<string, string>;
    }
  | {
      message: string | string[];
    }
);
