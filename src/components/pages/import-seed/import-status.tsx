import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogContent,
  AlertDialogDescription,
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
        <>
          <AlertDialogDescription>
            All records successfully imported
          </AlertDialogDescription>
          <AlertDialogAction onClick={onOk}>Ok</AlertDialogAction>
        </>
      ) : (
        <>
          <AlertDialogDescription>
            Import at {currentPercent}% completed
          </AlertDialogDescription>
          <Progress value={currentPercent} />
        </>
      )}
    </AlertDialogContent>
  </AlertDialog>
);
