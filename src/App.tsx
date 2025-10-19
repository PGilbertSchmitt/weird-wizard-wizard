import {useEffect } from "react";
import "./App.css";
import { useQuery } from "@tanstack/react-query";
import { dbSelect } from "./client";

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

  return (
    <main className="container">
      <h1>Weird Wizard <i>Wizard</i></h1>
    </main>
  );
}

export default App;
