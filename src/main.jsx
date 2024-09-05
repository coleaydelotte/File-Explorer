import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root")).render(
  // for some reason when running with react strict mode, it slows the app down by a lot
  // <React.StrictMode>
  //   <App />
  // </React.StrictMode>,

  // so I'm just going to run it without strict mode
  <App />
);
