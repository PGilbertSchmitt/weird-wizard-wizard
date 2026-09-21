import { cardStyle, pressStyle } from './styles';
import { cn } from '@/lib/utils';

type BaseButtonProps = React.DetailedHTMLProps<
  React.ButtonHTMLAttributes<HTMLButtonElement>,
  HTMLButtonElement
>;

interface ButtonProps extends BaseButtonProps {
  pressStyle?: boolean;
}

export const Button = ({pressStyle: pressStyleProp, ...props}: ButtonProps) => {
  const adjustedPressStyle =
    (pressStyleProp ?? true) ? pressStyle : 'shadow-0 shadow-none';
  return (
    <button
      {...props}
      className={cn(
        cardStyle,
        props.disabled ? 'saturate-0' : adjustedPressStyle,
        'cursor-pointer',
        props.className,
      )}
    >
      {props.children}
    </button>
  );
};
