import { toPairs } from 'ramda';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/neo/alert-dialog';
import { ImportError } from '@/lib/import-data/import-error';

interface ExtractErrorAlertProps {
  error: ImportError;
  canContinue: boolean;
  onOk: () => void;
  onCancel: () => void;
}

export const ExtractErrorAlert = ({
  error,
  canContinue,
  onOk,
  onCancel,
}: ExtractErrorAlertProps) => {
  return (
    <AlertDialog open>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{error.title}</AlertDialogTitle>
        </AlertDialogHeader>
        <RenderError error={error} />
        <AlertDialogFooter>
          {canContinue ? (
            <>
              <AlertDialogAction onClick={onOk}>Continue</AlertDialogAction>
              <AlertDialogCancel onClick={onCancel}>Cancel</AlertDialogCancel>
            </>
          ) : (
            <AlertDialogCancel onClick={onCancel}>Ok</AlertDialogCancel>
          )}
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
};

const RenderError = ({ error }: { error: ImportError }) => {
  if ('body' in error) {
    return (
      <>
        {toPairs(error.body).map(([key, msg]) => (
          <AlertDialogDescription key={key}>
            <b>{key}</b>: {msg}
          </AlertDialogDescription>
        ))}
      </>
    );
  } else if (typeof error.message === 'string') {
    return <AlertDialogDescription>{error.message}</AlertDialogDescription>;
  } else {
    return (
      <ul>
        {error.message.map((msg) => (
          <li key={msg}>{msg}</li>
        ))}
      </ul>
    );
  }
};
