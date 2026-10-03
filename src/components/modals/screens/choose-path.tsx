import { FullCharacter } from '@/types/character';
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from '@/components/ui/neo/tabs';
import { useCategoriesForKind } from '@/api/paths';
import { PathsByCategory } from '@/components/pages/catalogue/paths-by-category';
import { cn } from '@/lib/utils';
import { useSetPath } from '@/api/characters';
import { useModal } from '@/hooks/modal';

interface ChoosePathProps {
  character: FullCharacter;
  kind: 'Expert' | 'Master';
}

export const ChoosePath = ({ character, kind }: ChoosePathProps) => {
  const { data: categories } = useCategoriesForKind(kind);
  const { mutateAsync: setPath } = useSetPath(character.id);
  const { popNonErrorModal, pushError } = useModal();

  if (!categories) {
    return <div className={cn('w-full')} />;
  }

  return (
    <Tabs defaultValue={categories[0]}>
      <TabsList className="w-full flex items-start h-fit">
        {categories.map((category) => (
          <TabsTrigger key={category} value={category}>
            {category} Paths
          </TabsTrigger>
        ))}
      </TabsList>
      {categories.map((category) => (
        <TabsContent key={category} value={category}>
          <PathsByCategory
            kind={kind}
            category={category}
            onSelect={(pathId, _pathName, _ancestryId) => {
              setPath(pathId)
                .then(popNonErrorModal)
                .catch((err) => {
                  popNonErrorModal();
                  pushError(err.toString());
                });
            }}
            selectedId={-1}
          />
        </TabsContent>
      ))}
    </Tabs>
  );
};
