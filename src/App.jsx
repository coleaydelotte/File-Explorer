import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Box, Typography } from "@mui/material";

function App() {
  const [dirsToPrint, setDirsToPrint] = useState([]);

  async function testForwardFiles() {
    let path = 'C:\\Users\\Colea\\Desktop\\CodingProjects\\Rust\\File-Explorer\\src';
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
    <Box
      className="container"
      display={"flex"}
      alignItems={"center"}
      justifyContent={"center"}
      width={"100%"}
      height={"100%"}
    >
      {dirsToPrint.map((item, index) => (
        <Box
          className="mapBox"
          key={index}
          alignItems="center"
          border={1}
          borderColor={"primary.main"}
          margin={1}
          width="25%"
          justifyContent={"center"}
        >
          <Typography
            justifyContent="center"
          >
            {item}
          </Typography>
        </Box>
      ))}
    </Box>
  );
}

export default App;
