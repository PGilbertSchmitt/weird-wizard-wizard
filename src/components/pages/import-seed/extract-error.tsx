import { toPairs } from 'ramda';
import { ExtractError } from '../../../lib/import-data/unzip';
import {
  AlertDialog,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog';

interface ExtractErrorAlertProps {
  error: ExtractError;
  onOk: () => void;
}

export const ExtractErrorAlert = ({ error, onOk }: ExtractErrorAlertProps) => (
  <AlertDialog open>
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>{error.title}</AlertDialogTitle>
      </AlertDialogHeader>
      <RenderError error={error} />
      <AlertDialogCancel onClick={onOk}>Ok</AlertDialogCancel>
    </AlertDialogContent>
  </AlertDialog>
);

const RenderError = ({ error }: { error: ExtractError }) => {
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
      <AlertDialogDescription>
        <ul>
          {error.message.map((msg) => (
            <li key={msg}>{msg}</li>
          ))}
        </ul>
      </AlertDialogDescription>
    );
  }
};
