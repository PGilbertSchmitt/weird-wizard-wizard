import './App.css';
import { BrowserRouter, Route, Routes } from 'react-router';
import { ImportSeed } from './components/pages/import-seed';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
// import { Home } from './components/pages/home';
// import { Tome } from './components/pages/tome/tradition-index';
// import { TraditionPage } from './components/pages/tome/tradition-page';
// import { SeedDropzone } from "./dropzone";

export const client = new QueryClient();

const App = () => (
  <QueryClientProvider client={client}>
    <BrowserRouter>
      <Routes>
        {/* <Route path="/" Component={Home} />
        <Route path="/tome">
          <Route index Component={Tome} />
          <Route path=":traditionId" Component={TraditionPage} />
        </Route>
        <Route path="/paths" element={<p>paths</p>} />
        <Route path="/seed" Component={ImportSeed} /> */}
        <Route path="/" Component={ImportSeed} />
      </Routes>
    </BrowserRouter>
  </QueryClientProvider>
);

export default App;
