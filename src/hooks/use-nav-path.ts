import { useLocation } from 'react-router';

export const useNavPath = () => {
  const { pathname } = useLocation();
  const headlessPath =
    pathname.charAt(0) === '/' ? pathname.slice(1) : pathname;
  return headlessPath.split('/');
};
