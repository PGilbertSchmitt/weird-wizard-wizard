import { cn } from '@/lib/utils';

interface ParagraphProps {
  children: React.ReactNode;
  size?: 'sm' | 'lg';
}

export const Paragraph = ({ children, size }: ParagraphProps) => (
  <p className={cn('text-justify indent-6 my-3', size && `text-${size}`)}>
    {children}
  </p>
);
