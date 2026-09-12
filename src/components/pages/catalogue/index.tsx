import { usePathIndex } from '@/api/paths';
import { PathKind } from '@/types/etc';
import { PathIndexItem } from '@/types/path';
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from '@/components/ui/neo/tabs';
import { useMemo } from 'react';
import { keys, toPairs } from 'ramda';
import { PathsByCategory } from './paths-by-category';
import { Ancestries } from './ancestries';
import { AnimatePresence, motion } from 'motion/react';

type PathCollection = Record<PathKind, Record<string, PathIndexItem[]>>;

export const Catalogue = () => {
  const { data: rawPaths } = usePathIndex();

  const pathCollection: PathCollection = useMemo(() => {
    const collection: PathCollection = {
      Novice: {},
      Expert: {},
      Master: {},
    };
    if (rawPaths === undefined) {
      return collection;
    }

    for (const path of rawPaths) {
      const kind = path.path_kind;
      const category = path.category;

      collection[kind][category] ||= [];
      collection[kind][category].push(path);
    }

    return collection;
  }, [rawPaths]);

  return (
    <div>
      <h1>Catalogue of Paths</h1>

      <AnimatePresence>
        <Tabs defaultValue="Novice Generic Path">
          <TabsList className="border-none w-full flex items-start h-fit">
            <TabTriggers kind="Novice" byKind={pathCollection.Novice} />
            <TabTriggers kind="Expert" byKind={pathCollection.Expert} />
            <TabTriggers kind="Master" byKind={pathCollection.Master} />
          </TabsList>
          <TabsContent key="Ancestries" value="Ancestries">
            <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }}>
              <Ancestries />
            </motion.div>
          </TabsContent>
          {toPairs(pathCollection).flatMap(([kind, categoryMap]) =>
            keys(categoryMap).flatMap((cat) => {
              const contentKey = `${kind} ${cat}`;
              return (
                <TabsContent key={contentKey} value={contentKey}>
                  <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }}>
                    <PathsByCategory kind={kind} category={cat} />
                  </motion.div>
                </TabsContent>
              );
            }),
          )}
        </Tabs>
      </AnimatePresence>
    </div>
  );
};

interface TriggerListProps {
  kind: PathKind;
  byKind: Record<string, PathIndexItem[]>;
}

const TabTriggers = ({ kind, byKind }: TriggerListProps) => (
  <div className="flex flex-col">
    {kind === 'Novice' && (
      <TabsTrigger key="Ancestries" value="Ancestries">
        Ancestries
      </TabsTrigger>
    )}
    {keys(byKind).map((cat) => {
      const key = `${kind} ${cat}`;

      return (
        <TabsTrigger key={key} value={key}>
          {key}
        </TabsTrigger>
      );
    })}
  </div>
);
