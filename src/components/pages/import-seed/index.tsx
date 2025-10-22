import { Paragraph } from '@/components/ui/paragraph';
import { cn } from '@/lib/utils';
import { Dropzone } from './dropzone';

export const ImportSeed = () => (
  <>
    <h1>Import Static Data</h1>
    <div className={cn('flex flex-col items-center max w-100')}>
      <Paragraph>
        Static data refers to <i>Shadow of the Weird Wizard</i>-specific
        information that is needed to create a character. This includes things
        like ancestries, paths, traditions, and spells. This tool does not ship
        with this data because it's not my data to distribute (it belongs to
        Robert J. Schwalb and{' '}
        <a className="default" href="https://schwalbentertainment.com/">
          Schwalb Entertainment
        </a>
        ).
      </Paragraph>
      <Paragraph>
        If you have zip file of the seed data, you can drop it off here. If you
        don't have one yet, you can create one if you have the{' '}
        <a
          className="default"
          href="https://schwalbentertainment.com/shadow-of-the-weird-wizard/"
        >
          SofWW rule book
        </a>{' '}
        and a bunch of time on your hands.
      </Paragraph>

      <Dropzone />
    </div>
  </>
);
