import { useTraditions } from '@/api/magic';
import { TraditionIndexItem } from '@/types/magic';

interface WithTraditionsProps {
  onRender: (traditions: Array<TraditionIndexItem>) => React.ReactNode;
}

export const WithTraditions = ({ onRender }: WithTraditionsProps) => {
  const { data: traditions } = useTraditions();
  if (traditions) {
    return onRender(traditions);
  } else {
    return null;
  }
};
