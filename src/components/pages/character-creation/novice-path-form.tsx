import { PathsByCategory } from '../catalogue/paths-by-category';
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from '@/components/ui/neo/tabs';

interface NovicePathFormProps {
  noviceId: number;
  ancestryLocked: boolean | null;
  onSelect: (
    pathId: number,
    pathName: string,
    ancestryId: number | null,
  ) => void;
}

export const NovicePathForm = ({
  noviceId,
  ancestryLocked,
  onSelect,
}: NovicePathFormProps) => (
  <Tabs defaultValue={ancestryLocked ? 'Ancestries' : 'Generic Path'}>
    <TabsList className="border-none w-full flex items-start h-fit">
      <TabsTrigger value="Generic Path">Generic Novice Paths</TabsTrigger>
      <TabsTrigger value="Ancestries">Ancestry-based Novice Paths</TabsTrigger>
    </TabsList>
    <TabsContent value="Ancestries">
      <PathsByCategory
        kind="Novice"
        category="Ancestry-locked Path"
        selectedId={noviceId}
        onSelect={onSelect}
      />
    </TabsContent>
    <TabsContent value="Generic Path">
      <PathsByCategory
        kind="Novice"
        category="Generic Path"
        selectedId={noviceId}
        onSelect={onSelect}
      />
    </TabsContent>
  </Tabs>
);
