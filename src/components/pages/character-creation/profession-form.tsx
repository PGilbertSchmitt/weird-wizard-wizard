import { useProfessionIndex } from '@/api/professions';
import { Button } from '@/components/ui/button';
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/neo/accordion';
import { cn } from '@/lib/utils';
import { FullProfessionCategory } from '@/types/other_info';
import { useMemo } from 'react';

interface ProfessionFormProps {
  selected: number | null;
  onSelect: (id: number, name: string) => void;
}

export const ProfessionForm = ({ selected, onSelect }: ProfessionFormProps) => {
  const { data: professionCategories } = useProfessionIndex();

  const selectedProfession = useMemo(() => {
    const profId = selected ?? -1;
    for (const category of professionCategories || []) {
      for (const profession of category.professions) {
        if (profession.id === profId) {
          return profession;
        }
      }
    }
  }, [selected, professionCategories]);

  if (professionCategories === undefined) {
    return null;
  }

  return (
    <div>
      <div className="text-center mb-4">
        <p>
          <b>Selected:</b>{' '}
          {selectedProfession ? selectedProfession.name : 'None'}
        </p>
      </div>

      <div className="w-dvw max-w-250 px-4">
        <Accordion type="single" collapsible>
          {professionCategories.map((category) => (
            <Category
              key={category.id}
              selected={selected}
              onSelect={onSelect}
              category={category}
            />
          ))}
        </Accordion>
      </div>
    </div>
  );
};

interface CategoryProps extends ProfessionFormProps {
  category: FullProfessionCategory;
}

const Category = ({ selected, onSelect, category }: CategoryProps) => {
  return (
    <>
      <AccordionItem
        value={category.name}
        className={cn(
          'rounded-none border-y first:border-t-2 last:border-b-2 first:rounded-t-base last:rounded-b-base',
        )}
      >
        <AccordionTrigger>
          {category.name} - {category.description}
        </AccordionTrigger>
        <AccordionContent className="p-0">
          <table>
            <tbody>
              {category.professions.map((prof) => (
                <tr key={prof.id}>
                  <th className="p-2">{prof.name}</th>
                  <td className="p-2">
                    <Button
                      className="p-1"
                      pressStyle={false}
                      disabled={selected === prof.id}
                      onClick={() => onSelect(prof.id, prof.name)}
                    >
                      Pick
                    </Button>
                  </td>
                  <td className="p-2">{prof.description}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </AccordionContent>
      </AccordionItem>
    </>
  );
};
