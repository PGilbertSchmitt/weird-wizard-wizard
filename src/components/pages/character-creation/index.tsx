import { useMemo, useState } from 'react';
import { Button } from '@/components/ui/button';
import { ProfessionForm } from './profession-form';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { cn } from '@/lib/utils';
import { Input } from '@/components/ui/neo/input';

interface CharacterInfo {
  name: string;
  professionId: number;
  novicePathId: number;
  ancestryId: number;
}

const Step = {
  NAME: 0,
  PROFESSION: 1,
  NOVICE_PATH: 2,
  ANCESTRY: 3,
} as const;
type StepValue = (typeof Step)[keyof typeof Step];

const stepInstruction = (step: StepValue) => {
  switch (step) {
    case Step.NAME:
      return 'Choose a name';
    case Step.PROFESSION:
      return 'Pick a profession';
    case Step.NOVICE_PATH:
      return 'Choose a novice path';
    case Step.ANCESTRY:
      'Choose an ancestry';
  }
};

interface FormData {
  name: string;
  professionId: number | null;
  noviceId: number | null;
  ancestryLocked: boolean | null;
  ancestryId: number | null;
}

export const CharacterCreationPage = () => {
  const [characterInfo, setCharacterInfo] = useState<FormData>({
    name: '',
    professionId: null,
    ancestryId: null,
    ancestryLocked: null,
    noviceId: null,
  });

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
  }, [characterInfo]);

  const [curStep, setCurStep] = useState<StepValue>(Step.NAME);

  const nextStep = () => setCurStep((curStep + 1) as StepValue);
  const prevStep = () => setCurStep((curStep - 1) as StepValue);

  return (
    <>
      <div className={cn('flex flex-col items-center w-fit m-auto')}>
        <div className="text-center">
          <h1>Create a new Character</h1>
        </div>

        <div className={cn('flex flex-row justify-between mb-10 w-100 gap-10')}>
          <Button
            className={cn('p-1')}
            disabled={curStep === Step.NAME}
            onClick={prevStep}
          >
            <ChevronLeft strokeWidth="1px" size="14px" />
          </Button>
          <p>{stepInstruction(curStep)}</p>
          <Button
            className={cn('p-1')}
            disabled={curStep === maxStep}
            onClick={nextStep}
          >
            <ChevronRight strokeWidth="1px" size="14px" />
          </Button>
        </div>
      </div>

      {(() => {
        switch (curStep) {
          case Step.NAME:
            return (
              <>
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
              </>
            );
          case Step.PROFESSION:
            return (
              <ProfessionForm
                selected={characterInfo.professionId}
                onSelect={(id) => {
                  setCharacterInfo({
                    ...characterInfo,
                    professionId: id,
                  });
                  nextStep();
                }}
              />
            );
        }
      })()}
    </>
  );
};
