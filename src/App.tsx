import './App.css';
import { Route, Routes } from 'react-router';
import { ImportSeed } from './components/pages/import-seed';
import { Home } from './components/pages/home';
import { Tome } from './components/pages/tome/tradition-index';
import { TraditionPage } from './components/pages/tome/tradition-page';

const App = () => (
  <Routes>
    <Route path="/" Component={Home} />
    <Route path="/tome">
      <Route index Component={Tome} />
      <Route path=":traditionId" Component={TraditionPage} />
    </Route>
    <Route path="/paths" element={<p>paths</p>} />
    <Route path="/seed" Component={ImportSeed} />
  </Routes>
);

export default App;
