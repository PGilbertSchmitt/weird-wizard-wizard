import { cn } from '@/lib/utils';

interface ParagraphProps {
  children: React.ReactNode;
}

export const Paragraph = ({ children }: ParagraphProps) => (
  <p className={cn('text-justify indent-6 my-3')}>{children}</p>
);
