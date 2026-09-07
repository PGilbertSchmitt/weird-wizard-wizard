import { cn } from '@/lib/utils';

interface ParagraphProps {
  children: React.ReactNode;
  className?: string;
  size?: 'sm' | 'lg';
}

export const Paragraph = ({ children, size, className }: ParagraphProps) => (
  <p className={cn('text-justify indent-6 my-3', size && `text-${size}`, className)}>
    {children}
  </p>
);
