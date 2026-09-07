import { useCharacterIndex } from '@/api/characters';
import { Button } from '@/components/ui/button';
import { cardStyle, pressStyle } from '@/components/ui/styles';
import { cn } from '@/lib/utils';
import { Link } from 'react-router';

export const Home = () => {
  const { data: characters } = useCharacterIndex();

  console.log('Characters:', characters);

  const moreCharacters = [
    ...(characters || []),
    ...(characters || []),
    ...(characters || []),
  ];

  return (
    <div>
      <h1>Characters</h1>

      <div className={cn('w-140 flex flex-col items-center')}>
        {moreCharacters?.map((character) => {
          // const paths = [character.novice_path, character.expert_path, character.master_path];
          const paths: Array<[string, string]> = [
            [character.novice_path, 'Novice'],
          ];
          if (character.expert_path) {
            paths.push([character.expert_path, 'Expert']);
          }
          if (character.master_path) {
            paths.push([character.master_path, 'Master']);
          }
          return (
            <Link
              className={cn(cardStyle, pressStyle, 'my-2 w-full')}
              key={character.id}
              to={`/character/${character.id}`}
            >
              <div className="flex flex-row justify-between">
                <div className="text-left">
                  <h2>{character.name}</h2>
                  <p>
                    level {character.level} {character.ancestry}
                  </p>
                </div>
                <div className="text-right">
                  {paths.map((path) => (
                    <p>
                      <b>{path[0]}</b>
                      <span>&nbsp;({path[1]})</span>
                    </p>
                  ))}
                </div>
              </div>
            </Link>
          );
        })}

        <Button className="my-2 w-fit">Create Character</Button>
      </div>
    </div>
  );
};
