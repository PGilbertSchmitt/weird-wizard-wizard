import { useMemo, useState } from 'react';
import { Button } from '@/components/ui/button';
import { ProfessionForm } from './profession-form';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { cn } from '@/lib/utils';
import { Input } from '@/components/ui/neo/input';
import { NovicePathForm } from './novice-path-form';
import { AnimatePresence, motion } from 'motion/react';
import { Ancestries } from '../catalogue/ancestries';
import { LockedAncestry } from './locked-ancestry';
import { Summary } from './summary';
import { useCreateCharacter } from '@/api/characters';
import { useNavigate } from 'react-router';
import { ChooseScores, Scores } from './choose-scores';

const Step = {
  NAME: 0,
  PROFESSION: 1,
  NOVICE_PATH: 2,
  ANCESTRY: 3,
  SCORES: 4,
  SUMMARY: 5,
} as const;
type StepValue = (typeof Step)[keyof typeof Step];

const stepInstruction = (step: StepValue, ancestryLocked: boolean | null) => {
  switch (step) {
    case Step.NAME:
      return 'Choose a name';
    case Step.PROFESSION:
      return 'Choose a profession';
    case Step.NOVICE_PATH:
      return 'Choose a novice path';
    case Step.ANCESTRY:
      return ancestryLocked ? 'Confirm ancestry' : 'Choose an ancestry';
    case Step.SCORES:
      return 'Decide ability scores';
    case Step.SUMMARY:
      return 'Summary';
  }
};

interface FormData {
  name: string;
  professionId: number | null;
  professionName: string;
  noviceId: number | null;
  noviceName: string;
  ancestryLocked: boolean | null;
  ancestryId: number | null;
  ancestryName: string;
  scores: Scores | null;
}

export const CharacterCreationPage = () => {
  const [characterInfo, setCharacterInfo] = useState<FormData>({
    name: '',
    professionId: null,
    ancestryId: null,
    ancestryLocked: null,
    noviceId: null,
    professionName: '',
    noviceName: '',
    ancestryName: '',
    scores: null,
  });

  const { mutateAsync } = useCreateCharacter();

  const navigate = useNavigate();

  const maxStep = useMemo(() => {
    if (characterInfo.name.length === 0) {
      return Step.NAME;
    }

    if (characterInfo.professionId === null) {
      return Step.PROFESSION;
    }

    if (characterInfo.noviceId === null) {
      return Step.NOVICE_PATH;
    }

    if (characterInfo.ancestryId === null) {
      return Step.ANCESTRY;
    }

    if (characterInfo.scores === null) {
      return Step.SCORES;
    }

    return Step.SUMMARY;
  }, [characterInfo]);

  const [curStep, setCurStep] = useState<StepValue>(Step.NAME);

  const scrollToTop = () => {
    window.scrollTo(0, 0);
  };
  const nextStep = () => {
    scrollToTop();
    setCurStep((curStep + 1) as StepValue);
  };
  const prevStep = () => {
    scrollToTop();
    setCurStep((curStep - 1) as StepValue);
  };

  return (
    <>
      <div className={cn('flex flex-col items-center w-fit m-auto')}>
        <div className="text-center">
          <h1>Create a new Character</h1>
        </div>

        <div
          className={cn(
            'flex flex-row justify-between items-center mb-10 w-100 gap-10',
          )}
        >
          <Button
            className={cn('p-1', curStep === Step.NAME && 'opacity-0')}
            disabled={curStep === Step.NAME}
            onClick={prevStep}
          >
            <ChevronLeft strokeWidth="1px" size="14px" />
          </Button>
          <p>{stepInstruction(curStep, characterInfo.ancestryLocked)}</p>
          <Button
            className={cn('p-1', curStep === Step.SUMMARY && 'opacity-0')}
            disabled={curStep === maxStep}
            onClick={nextStep}
          >
            <ChevronRight strokeWidth="1px" size="14px" />
          </Button>
        </div>
      </div>

      <AnimatePresence>
        {(() => {
          switch (curStep) {
            case Step.NAME:
              return (
                <motion.div
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  key={Step.NAME}
                >
                  <Input
                    id="character-name"
                    value={characterInfo.name}
                    placeholder="Character Name"
                    onChange={(e) =>
                      setCharacterInfo({
                        ...characterInfo,
                        name: e.target.value,
                      })
                    }
                  />
                </motion.div>
              );
            case Step.PROFESSION:
              return (
                <motion.div
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  key={Step.PROFESSION}
                >
                  <ProfessionForm
                    selected={characterInfo.professionId}
                    onSelect={(id, name) => {
                      setCharacterInfo({
                        ...characterInfo,
                        professionId: id,
                        professionName: name,
                      });
                      nextStep();
                    }}
                  />
                </motion.div>
              );
            case Step.NOVICE_PATH:
              return (
                <motion.div
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  key={Step.NOVICE_PATH}
                >
                  <NovicePathForm
                    noviceId={characterInfo.noviceId ?? -1}
                    ancestryLocked={characterInfo.ancestryLocked}
                    onSelect={(noviceId, noviceName, ancestryId) => {
                      setCharacterInfo({
                        ...characterInfo,
                        noviceId,
                        noviceName,
                        ancestryId,
                        ancestryLocked: ancestryId !== null,
                      });
                      nextStep();
                    }}
                  />
                </motion.div>
              );
            case Step.ANCESTRY:
              return (
                <motion.div
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  key={Step.NOVICE_PATH}
                >
                  {characterInfo.ancestryLocked ? (
                    <LockedAncestry
                      ancestryId={characterInfo.ancestryId!}
                      setName={(name) => {
                        setCharacterInfo({
                          ...characterInfo,
                          ancestryName: name,
                        });
                      }}
                    />
                  ) : (
                    <Ancestries
                      selectedId={characterInfo.ancestryId ?? -1}
                      onSelect={(ancestryId, ancestryName) => {
                        setCharacterInfo({
                          ...characterInfo,
                          ancestryId,
                          ancestryName,
                        });
                        nextStep();
                      }}
                    />
                  )}
                </motion.div>
              );
            case Step.SCORES:
              return (
                <motion.div
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  key={Step.NOVICE_PATH}
                >
                  <ChooseScores
                    novicePathId={characterInfo.noviceId || -1}
                    onConfirm={(scores) => {
                      setCharacterInfo({
                        ...characterInfo,
                        scores,
                      });
                      nextStep();
                    }}
                  />
                </motion.div>
              );
            case Step.SUMMARY:
              return (
                <motion.div
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  key={Step.NOVICE_PATH}
                >
                  <Summary
                    name={characterInfo.name}
                    professionName={characterInfo.professionName}
                    novicePathName={characterInfo.noviceName}
                    ancestryName={characterInfo.ancestryName}
                    ancestryLocked={characterInfo.ancestryLocked || false}
                    scores={characterInfo.scores!}
                    onConfirm={async () => {
                      const id = await mutateAsync({
                        name: characterInfo.name,
                        profession_id: characterInfo.professionId!,
                        novice_path_id: characterInfo.noviceId!,
                        ancestry_id: characterInfo.ancestryId!,
                        strength: characterInfo.scores?.strength!,
                        agility: characterInfo.scores?.agility!,
                        intellect: characterInfo.scores?.intellect!,
                        will: characterInfo.scores?.will!,
                      });
                      navigate(`/character/${id}`);
                    }}
                  />
                </motion.div>
              );
          }
        })()}
      </AnimatePresence>
    </>
  );
};
