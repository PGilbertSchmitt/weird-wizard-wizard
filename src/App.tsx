import './App.css';
import { BrowserRouter, Route, Routes } from 'react-router';
import { ImportSeed } from './components/pages/import-seed';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Layout } from './components/layout/layout';
import { Home } from './components/pages/home';
import { ThemeProvider } from './components/providers/theme-provider';
import { CharacterPage } from './components/pages/characters';
import { Tome } from './components/pages/tome';
import { TraditionPage } from './components/pages/tome/tradition-page';
import { TooltipProvider } from './components/ui/neo/tooltip';
import { Catalogue } from './components/pages/catalogue';
import { CharacterCreationPage } from './components/pages/character-creation';
// import { Tome } from './components/pages/tome/tradition-index';
// import { TraditionPage } from './components/pages/tome/tradition-page';
// import { SeedDropzone } from "./dropzone";

export const client = new QueryClient();

const App = () => (
  <QueryClientProvider client={client}>
    <BrowserRouter>
      <ThemeProvider>
        <TooltipProvider>
          <Layout>
            <Routes>
              <Route path="/" Component={Home} />
              <Route path="/character/:id" Component={CharacterPage} />
              <Route
                path="/character-creation"
                Component={CharacterCreationPage}
              />
              <Route path="/tome">
                <Route index Component={Tome} />
                <Route path=":traditionId" Component={TraditionPage} />
              </Route>
              <Route path="/Catalogue" Component={Catalogue} />
              <Route path="/import" Component={ImportSeed} />
            </Routes>
          </Layout>
        </TooltipProvider>
      </ThemeProvider>
    </BrowserRouter>
  </QueryClientProvider>
);

export default App;
