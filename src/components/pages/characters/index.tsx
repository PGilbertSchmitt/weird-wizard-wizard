import { useParams } from 'react-router';

export const CharacterPage = () => {
  const params = useParams();
  const id = parseInt(params['id'] || '-1');

  return <h1>You are somebody with id {id}</h1>;
};
