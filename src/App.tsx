import { useContext, useEffect } from 'react';
import './App.css';
import { useQuery } from '@tanstack/react-query';
import { dbSelect } from './client';
import { Route, Routes } from 'react-router';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
  DropdownMenuItem,
} from './components/ui/dropdown-menu';
import { ThemeProviderContext } from './contexts/theme-context';
import { times } from 'ramda';
import { ImportSeed } from './components/pages/import-seed';
import { Button } from './components/ui/button';

function App() {
  const { data, isFetched } = useQuery({
    queryKey: ['all-levels'],
    queryFn: () => {
      return dbSelect('SELECT * FROM levels');
    },
  });

  useEffect(() => {
    if (isFetched) {
      console.log('level data:', data);
    }
  }, [isFetched]);

  const { setTheme } = useContext(ThemeProviderContext);

  return (
    <Routes>
      <Route
        path="/"
        element={
          <main className="container">
            <h1>
              Weird Wizard <i>Wizard</i>
            </h1>

            {times(
              (i) => (
                <div
                  key={i}
                  className="rounded-base border-border bg-main text-main-foreground shadow-shadow border-2 p-4 mb-5 last:mb-0"
                >
                  <p>Lorem Ipsum is a great book</p>
                </div>
              ),
              3,
            )}

            <Button>Click me!</Button>
          </main>
        }
      />
      <Route path="/seed" Component={ImportSeed} />
      <Route
        path="/themes"
        element={
          <main className="container">
            <h1>Seed static tables</h1>
            <DropdownMenu>
              <DropdownMenuTrigger>Open</DropdownMenuTrigger>
              <DropdownMenuContent className="w-56">
                <DropdownMenuItem onSelect={() => setTheme('dark-red')}>
                  Dark Red
                </DropdownMenuItem>
                <DropdownMenuItem onSelect={() => setTheme('light-red')}>
                  Light Red
                </DropdownMenuItem>
                <DropdownMenuItem onSelect={() => setTheme('dark-cyan')}>
                  Dark Cyan
                </DropdownMenuItem>
                <DropdownMenuItem onSelect={() => setTheme('light-cyan')}>
                  Light Cyan
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </main>
        }
      />
    </Routes>
  );
}

export default App;
