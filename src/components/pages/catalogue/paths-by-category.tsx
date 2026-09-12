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
import { useMemo } from 'react';
import { LevelSection } from './level-section';
import { Paragraph } from '@/components/ui/paragraph';
import { useCollapseState } from '@/hooks/use-collapse-state';

interface PathsByCategoryProps {
  kind: string;
  category: string;
  selectedId?: number;
  onSelect?: (
    pathId: number,
    pathName: string,
    ancestryId: number | null,
  ) => void;
}

export const PathsByCategory = ({
  kind,
  category,
  onSelect,
  selectedId,
}: PathsByCategoryProps) => {
  const { isFetched, data: rawPaths } = usePathsForCategory(kind, category);
  const paths = rawPaths || [];

  const ids = useMemo(
    () => (isFetched ? paths.map((p) => p.id) : []),
    [isFetched, paths],
  );

  const { isCollapsed, toggleCollapse, toggleAll, allCollapsed } =
    useCollapseState(ids);

  if (!isFetched) {
    return null;
  }

  return (
    <div className={cn('w-dvw max-w-250 px-4')}>
      <Button className="p-1" onClick={toggleAll}>
        {allCollapsed ? (
          <ListChevronsUpDown strokeWidth="1px" size="14px" />
        ) : (
          <ListChevronsDownUp strokeWidth="1px" size="14px" />
        )}
      </Button>

      {paths.map((item) => {
        const collapsed = isCollapsed(item.id);

        return (
          <StaticCard key={item.id} className={cn('my-4 p-0')}>
            <div
              className="flex flex-row justify-between items-center p-2 cursor-pointer"
              onClick={() => toggleCollapse(item.id)}
            >
              <div className={cn('flex flex-row gap-4')}>
                <h2>{item.name}</h2>
                {selectedId === item.id && <p>(Selected)</p>}
              </div>
              {collapsed ? (
                <ChevronDown strokeWidth="1px" size="18px" />
              ) : (
                <ChevronUp strokeWidth="1px" size="18px" />
              )}
            </div>

            {!collapsed && (
              <>
                <Separator />

                <div className={cn('bg-secondary-background text-foreground')}>
                  {typeof selectedId === 'number' && selectedId !== item.id && (
                    <div className={cn('flex justify-center pt-2')}>
                      <Button
                        className={cn('py-0 shadow-0')}
                        onClick={() =>
                          onSelect &&
                          onSelect(item.id, item.name, item.ancestry_id)
                        }
                      >
                        Pick
                      </Button>
                    </div>
                  )}

                  <Paragraph className="m-0 p-4">{item.description}</Paragraph>

                  {item.levels.map((level) => (
                    <div key={level.id}>
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
