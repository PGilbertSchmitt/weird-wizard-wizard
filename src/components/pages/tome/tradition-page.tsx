import { Tabs, TabsList, TabsTrigger } from "@/components/ui/neo/tabs";
import { Paragraph } from "@/components/ui/paragraph";
import { getFullTradition } from "@/lib/db/magic";
import { cn } from "@/lib/utils";
import { TabsContent } from "@radix-ui/react-tabs";
import { useQuery } from "@tanstack/react-query";
import { isNil, isNotNil } from "ramda";
import { useEffect } from "react";
import { useParams } from "react-router";
import { SpellCard } from "./spell-card";
import { TalentCard } from "./talent-card";
import { SpecialInfo } from "./magic-utils";
import { Grid } from "@/components/ui/grid";

export const TraditionPage = () => {
  const { traditionId: traditionIdParam } = useParams();

  const { data: tradition, error, isError } = useQuery({
    queryKey: ['tradition', traditionIdParam],
    queryFn: async () => {
      const traditionId = parseInt(traditionIdParam || '');
      return await getFullTradition(traditionId);
    },
    enabled: isNotNil(traditionIdParam),
    retry: false,
  });

  // useEffect(() => {
  //   if (isFetched) {
  //     console.log(tradition);
  //   }
  // }, [isFetched, tradition]);

  useEffect(() => {
    if (isError) {
      console.error(error);
    }
  }, [isError, error]);

  if (isNil(tradition)) {
    return null;
  }

  return (
    <div className={cn('w-full')}>
      <h1>{tradition.name}</h1>
      <p><b>{tradition.blurb}</b></p>
      <Paragraph>{tradition.description}</Paragraph>
      {tradition.specialInfo && (
        <SpecialInfo text={tradition.specialInfo} />
      )}

      <Tabs defaultValue="talents" >
        <TabsList className="border-foreground">
          <TabsTrigger value="talents">Talents</TabsTrigger>
          <TabsTrigger value="novice">Novice Spells</TabsTrigger>
          <TabsTrigger value="expert">Expert Spells</TabsTrigger>
          <TabsTrigger value="master">Master Spells</TabsTrigger>
        </TabsList>
        <TabsContent value="talents">
          <Grid>
            {tradition.talents.map(talent => (
              <TalentCard key={talent.id} talent={talent} />
            ))}
          </Grid>
        </TabsContent>
        <TabsContent value="novice">
          <Grid>
            {tradition.noviceSpells.map(spell => (
              <SpellCard key={spell.id} spell={spell} />
            ))}
          </Grid>
        </TabsContent>
        <TabsContent value="expert">
          <Grid>
            {tradition.expertSpells.map(spell => (
              <SpellCard key={spell.id} spell={spell} />
            ))}
          </Grid>
        </TabsContent>
        <TabsContent value="master">
          <Grid>
            {tradition.masterSpells.map(spell => (
              <SpellCard key={spell.id} spell={spell} />
            ))}
          </Grid>
        </TabsContent>
      </Tabs>
    </div>
  );
};
