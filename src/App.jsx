import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Box, Typography, TextField, Button, Stack } from "@mui/material";
import {  ThemeProvider } from "@mui/material/styles";
import theme from "./theme";
import "./App.css";

function App() {
  const [os, setOs] = useState("");
  const [path, setPath] = useState("");
  const [dirsToPrint, setDirsToPrint] = useState([]);

  async function getForwardFiles() {
    if (os === "windows") {
      setPath(await invoke("format_path_for_windows", { path: path }));
    }
    let boolean = true;
    try {
        let result = await invoke("output_files_as_vector", { path: path, printFiles: boolean, os: os });
        setDirsToPrint(result);
    } catch (error) {
        setDirsToPrint(["Error: ", error]);
    }
  }

  async function retrieveOs() {
    try {
      const result = await invoke("get_os");
      setOs(result);
    } catch (error) {
      setOs("Error: " + error);
    }
  }

useEffect(() => {
  retrieveOs();
  // getForwardFiles();
}, []);

  return (
    <ThemeProvider theme={theme}>
      <Box
        className="container"
        display={"flex"}
        alignItems={"center"}
        justifyContent={"center"}
        width={"100%"}
        height={"100%"}
      >
        <form
          onSubmit={ (e) => {
              e.preventDefault();
              getForwardFiles();
            }
          }
        >
          <Stack
            spacing={2}
            directions="column"
          >
            <TextField
              className="textField"
              required
              label="Path"
              text-color="#f6f6f6"
              onChange={
                (e) => setPath(e.target.value)
              }
            />
            <Button type="submit">Submit</Button>
          </Stack>
        </form>
        {dirsToPrint.length > 0 && (
          dirsToPrint.map((item, index) => (
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
          ))
        )}
      </Box>
    </ThemeProvider>
  );
}

export default App;
