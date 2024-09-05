import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Box, Typography } from "@mui/material";

function App() {
  const [dirsToPrint, setDirsToPrint] = useState([]);

  async function testForwardFiles() {
    let path = '/Users/aydelottec';
    let boolean = true;
    try {
        let result = await invoke("output_files_as_vector", { path: path, printFiles: boolean });
        setDirsToPrint(result);
    } catch (error) {
        setDirsToPrint(["Error: ", error]);
    }
}

testForwardFiles();

  return (
    <div className="container">
      <p>Files:</p>
      <Box>
        {dirsToPrint.map((item, index) => (
          <Box
            className="mapBox"
            key={index}
            alignItems="center"
          >
            <Typography
              justifyContent="center"
            >
              {item}
            </Typography>
          </Box>
        ))}
      </Box>
    </div>
  );
}

export default App;
