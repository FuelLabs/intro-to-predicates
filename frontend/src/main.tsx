import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import { FuelProvider } from "@fuel-wallet/react";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <FuelProvider
      fuelConfig={{
        devMode: true,
      }}
    >
      <App />
    </FuelProvider>
  </React.StrictMode>
);
