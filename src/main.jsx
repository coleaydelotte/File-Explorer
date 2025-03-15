import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

// Had issues with React DOM and Tauri, so I had to use the createRoot method instead of the render method.
ReactDOM.createRoot(document.getElementById("root")).render(
  <App />
);
