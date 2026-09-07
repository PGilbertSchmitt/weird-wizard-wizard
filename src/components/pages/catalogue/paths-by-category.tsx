import { usePathsForCategory } from '@/api/paths';
import { Button } from '@/components/ui/button';
import { StaticCard } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import {
  ChevronDown,
  ChevronUp,
  ListChevronsDownUp,
  ListChevronsUpDown,
} from 'lucide-react';
import { all } from 'ramda';
import { useMemo, useState } from 'react';
import { LevelSection } from './level-section';
import { Paragraph } from '@/components/ui/paragraph';

interface PathsByCategoryProps {
  kind: string;
  category: string;
}

export const PathsByCategory = ({ kind, category }: PathsByCategoryProps) => {
  const { isFetched, data: rawPaths } = usePathsForCategory(kind, category);
  const paths = rawPaths || [];

  const [collapseState, setCollapsedState] = useState<Record<number, boolean>>(
    {},
  );

  const toggleCollapse = (id: number) => {
    setCollapsedState({
      ...collapseState,
      [id]: !collapseState[id],
    });
  };

  const allCollapsed = useMemo(() => {
    if (!isFetched) {
      return false;
    }
    return all(({ id }) => !!collapseState[id], paths);
  }, [isFetched, collapseState]);

  if (!isFetched) {
    return null;
  }

  return (
    <div className={cn('w-dvw max-w-250 px-4')}>
      <Button
        className="p-1"
        onClick={() => {
          setCollapsedState(
            paths.reduce(
              (acc, { id }) => ({
                ...acc,
                [id]: !allCollapsed,
              }),
              {},
            ),
          );
        }}
      >
        {allCollapsed ? (
          <ListChevronsUpDown strokeWidth="1px" size="14px" />
        ) : (
          <ListChevronsDownUp strokeWidth="1px" size="14px" />
        )}
      </Button>

      {paths.map((item) => {
        const collapsed = !!collapseState[item.id];

        return (
          <StaticCard key={item.id} className={cn('my-4 p-0')}>
            <div className="flex flex-row justify-between items-center p-2 cursor-pointer">
              <h2>{item.name}</h2>
              <Button className="p-1" onClick={() => toggleCollapse(item.id)}>
                {collapsed ? (
                  <ChevronDown strokeWidth="1px" size="14px" />
                ) : (
                  <ChevronUp strokeWidth="1px" size="14px" />
                )}
              </Button>
            </div>

            {!collapsed && (
              <>
                <Separator />
                <div className="bg-secondary-background text-foreground">
                  <Paragraph className="m-0 p-4">{item.description}</Paragraph>

                  {item.levels.map((level) => (
                    <div>
                      <Separator />
                      <LevelSection level={level} />
                    </div>
                  ))}
                </div>
              </>
            )}
          </StaticCard>
        );
      })}
    </div>
  );
};
