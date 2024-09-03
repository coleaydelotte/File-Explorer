<!-- @/src/.App.jsx -->

# If the Rust back end can pass a vector and Tauri automatically turns it into a array.

import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const invoke = window.__TAURI__.invoke
  const [osMsg, setOsMsg] = useState("");
  const [path, setPath] = useState("");
  const [dirsToPrint, setDirsToPrint] = useState([]);

  useEffect(() => {
    async function fetchOs() {
      const os = await invoke("get_os", {});
      setOsMsg("Operating System: " + os);
      setDirsToPrint(["Click the button below to get directories."]);
    }
    fetchOs();
  }, []);

  function getForwardDirectories(modifiedPath) {
    let bool = true;
    return invoke("output_files_as_vector", { path: modifiedPath, bool });
  }

  function testForwardFiles() {
    invoke("output_files_as_vector", { path: "C:\\Users\\", bool: false })
      .then((res) => {
        console.log("Result from testForwardFiles:", res);
        setDirsToPrint(res);
      })
      .catch((error) => {
        console.error("Error in testForwardFiles:", error);
        setDirsToPrint(["THIS IS AN ERROR MESSAGE"]);
      });
  }
  

  function handleGetDirectories(modifiedPath) {
    const dirs = getForwardDirectories(modifiedPath);
    setDirsToPrint(dirs);
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>
      <p>{osMsg}</p>
      <ul>
        {dirsToPrint.length > 0 ? (
          dirsToPrint.map((dir, index) => (
            <li key={index}>{dir}</li>
          ))
        ) : (
          <li>No directories found.</li>
        )}
      </ul>
    </div>
  );
}

export default App;
