import { useCharacterIndex } from '@/api/characters';

export const Home = () => {
  const { data: characters } = useCharacterIndex();

  console.log('Characters:', characters);

  return (
    <div>
      <h1>Characters</h1>
    </div>
  );
};
