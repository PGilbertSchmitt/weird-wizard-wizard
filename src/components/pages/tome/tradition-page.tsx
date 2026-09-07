import { useFullTradition } from '@/api/magic';
import { cn } from '@/lib/utils';
import { useParams } from 'react-router';
import { SpecialInfo } from './special-info';
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from '@/components/ui/neo/tabs';
import { MagicTalentCard } from './magic-talent-card';
import { SpellCard } from './magic-spell-card';

export const TraditionPage = () => {
  const traditionParam = useParams()['traditionId'] || '-1';
  const traditionId = parseInt(traditionParam);

  const { data: traditionData } = useFullTradition(traditionId);

  console.log('Tradition:', traditionData);

  if (!traditionData) {
    return null;
  }

  return (
    <div className={cn('flex flex-col gap-6 w-200')}>
      <div className={cn('text-center')}>
        <h1>{traditionData.name}</h1>
        <b>{traditionData.blurb}</b>
      </div>

      <p className={cn('text-justify')}>{traditionData.description}</p>

      <SpecialInfo specialInfo={traditionData.special_info} />

      <Tabs defaultValue="talents">
        <TabsList className="border-none w-full">
          <TabsTrigger value="talents">Talents</TabsTrigger>
          <TabsTrigger value="novice">Novice</TabsTrigger>
          <TabsTrigger value="expert">Expert</TabsTrigger>
          <TabsTrigger value="master">Master</TabsTrigger>
        </TabsList>
        <TabsContent value="talents" className={cn('flex flex-col gap-4')}>
          <h1>Magic Talents</h1>
          {traditionData.talents.map((talent) => (
            <MagicTalentCard talent={talent} />
          ))}
        </TabsContent>
        <TabsContent value="novice" className={cn('flex flex-col gap-4')}>
          <h1>Novice Spells</h1>
          {traditionData.novice_spells.map((spell) => (
            <SpellCard spell={spell} />
          ))}
        </TabsContent>
        <TabsContent value="expert" className={cn('flex flex-col gap-4')}>
          <h1>Expert Spells</h1>
          {traditionData.expert_spells.map((spell) => (
            <SpellCard spell={spell} />
          ))}
        </TabsContent>
        <TabsContent value="master" className={cn('flex flex-col gap-4')}>
          <h1>Master Spells</h1>
          {traditionData.master_spells.map((spell) => (
            <SpellCard spell={spell} />
          ))}
        </TabsContent>
      </Tabs>
    </div>
  );
};
