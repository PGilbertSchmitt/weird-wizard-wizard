import { toPairs } from "ramda";
import { ExtractError } from "../../../lib/import-data/unzip";
import { AlertDialog, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogHeader, AlertDialogTitle } from '@/components/ui/alert-dialog';

interface ExtractErrorAlertProps {
  error: ExtractError;
  onOk: () => void;
}

export const ExtractErrorAlert = ({ error, onOk }: ExtractErrorAlertProps) => (
  <AlertDialog open>
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>
          {error.title}
        </AlertDialogTitle>
      </AlertDialogHeader>
      <AlertDialogDescription>
        <RenderError error={error} />
      </AlertDialogDescription>
      <AlertDialogCancel onClick={onOk}>Ok</AlertDialogCancel>
    </AlertDialogContent>
  </AlertDialog>
);

const RenderError = ({ error }: { error: ExtractError }) => {
  if ('body' in error) {
    return (
      <>
        {toPairs(error.body).map(([key, msg]) => (
          <div key={key}>
            <b>{key}</b>: {msg}
          </div>
        ))}
      </>
    );
  } else if (typeof error.message === 'string') {
    return (
      <span>{error.message}</span>
    );
  } else {
    return (
      <ul>
        {error.message.map(msg => (
          <li key={msg}>{msg}</li>
        ))}
      </ul>
    );
  }
}
