import { useContext, useEffect, useState } from "react";
import "./App.css";
import { useQuery } from "@tanstack/react-query";
import { dbSelect } from "./client";
import { Route, Routes } from "react-router";
import { DropdownMenu, DropdownMenuContent, DropdownMenuTrigger, DropdownMenuItem } from "./components/ui/dropdown-menu";
import { ThemeProviderContext } from "./components/contexts/theme-context";

function App() {
  const { data, isFetched } = useQuery({
    queryKey: ['all-levels'],
    queryFn: () => {
      return dbSelect('SELECT * FROM levels');
    }
  });
  
  useEffect(() => {
    if (isFetched) {
      console.log('level data:', data);
    }
  }, [isFetched]);

  const { setTheme } = useContext(ThemeProviderContext);

  return (
    <Routes>
      <Route path="/" element={
        <main className="container">
          <h1>Weird Wizard <i>Wizard</i></h1>
        </main>
      }>
      </Route>
      <Route path="/seed" element={
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
      }>
      </Route>
    </Routes>
  );
}

export default App;
