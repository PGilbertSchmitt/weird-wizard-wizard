import { cn } from '@/lib/utils';
import { Link } from 'react-router';
import { cardStyle, pressStyle } from './styles';

interface CardProps {
  children: React.ReactNode;
  className?: string;
  onClick?: () => void;
}

export const StaticCard = ({ children, className, onClick }: CardProps) => (
  <div
    className={cn(cardStyle, onClick && pressStyle, className)}
    onClick={onClick}
  >
    {children}
  </div>
);

interface NavCardProps {
  children: React.ReactNode;
  className?: string;
  href: string;
}

export const NavCard = ({ children, href, className }: NavCardProps) => (
  <Link to={href} className={cn(cardStyle, pressStyle, className)}>
    {children}
  </Link>
);
