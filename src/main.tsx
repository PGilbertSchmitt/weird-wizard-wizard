import ReactDOM from "react-dom/client";
import App from "./App";
import { BrowserRouter } from "react-router";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  // Removed StrictMode because I use API calls in a reducer that I don't want
  // running twice.
  <BrowserRouter>
    <App />
  </BrowserRouter>,
);
