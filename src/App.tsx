import './App.css';
import { useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import { dbSelect } from './lib/db/client';
import { Route, Routes } from 'react-router';
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

  return (
    <Routes>
      <Route
        path="/"
        element={
          <>
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
          </>
        }
      />
      <Route path="/seed" Component={ImportSeed} />
    </Routes>
  );
}

export default App;
