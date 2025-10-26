import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog';
import { Progress } from '@/components/ui/progress';

interface ImportLoadProgressProps {
  currentItem: string;
  currentPercent: number;
  done: boolean;
  onOk: () => void;
}

export const ImportLoadProgress = ({
  currentItem,
  currentPercent,
  done,
  onOk,
}: ImportLoadProgressProps) => (
  <AlertDialog open>
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>
          {done ? 'Done!' : `Creating ${currentItem}`}
        </AlertDialogTitle>
      </AlertDialogHeader>
      {done ? (
        <AlertDialogAction onClick={onOk}>Ok</AlertDialogAction>
      ) : (
        <Progress value={currentPercent} />
      )}
    </AlertDialogContent>
  </AlertDialog>
);
