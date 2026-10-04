import { useTraditions } from '@/api/magic';
import { TraditionIndexItem } from '@/types/magic';
import { useMemo } from 'react';
import { Spinner } from '../ui/neo/spinner';

interface WithTraditionsProps {
  limitedTraditions?: string[];
  onRender: (traditions: Array<TraditionIndexItem>) => React.ReactNode;
}

export const WithTraditions = ({
  limitedTraditions,
  onRender,
}: WithTraditionsProps) => {
  const { data: allTraditions } = useTraditions();

  const traditions = useMemo(() => {
    if (!allTraditions) {
      return null;
    }
    const anyTradition =
      limitedTraditions === undefined || limitedTraditions.includes('ANY');
    return anyTradition
      ? allTraditions
      : allTraditions.filter((t) => limitedTraditions.includes(t.name));
  }, [allTraditions, limitedTraditions]);

  if (traditions) {
    return onRender(traditions);
  } else {
    return <Spinner />;
  }
};
