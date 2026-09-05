import { cardStyle, pressStyle } from './styles';
import { cn } from '@/lib/utils';

type BaseButtonProps = React.DetailedHTMLProps<
  React.ButtonHTMLAttributes<HTMLButtonElement>,
  HTMLButtonElement
>;

export const Button = (props: BaseButtonProps) => {
  return (
    <button {...props} className={cn(cardStyle, pressStyle, props.className)}>
      {props.children}
    </button>
  );
};
