import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Box, Typography, TextField, Button, Stack } from "@mui/material";
import { ThemeProvider } from "@mui/material";
import theme from "./theme"
import "./App.css";

function App() {
  const [os, setOs] = useState("");
  const [path, setPath] = useState("");
  const [dirsToPrint, setDirsToPrint] = useState([]);
  const [displayPath, setDisplayPath] = useState("")

  let forwardDirs = new Map();

  async function formatForOS () {
    try {
      if (os === "windows") {
        setPath(await invoke("format_path_for_windows", { path: path }));
      }
    } catch (error) {
      setPath("Error: " + error);
    }
  }

  async function getForwardFiles() {
    let boolean = true;
    try {
      let result = await invoke("output_files_as_vector", { path: path, printFiles: boolean});
      setDirsToPrint(result);
    } catch (error) {
      setDirsToPrint(["Error: ", error]);
    }
  }

  async function openFile(index) {
    try {
      await invoke("open_file", { index: index, path: path });
    } catch (error) {
      setPath("Error: " + error);
    }
  }

  async function stepUp() {
    try {
      let newPath = await invoke("step_up", { path: path });
      setDisplayPath(newPath);
      setPath(newPath);
    } catch (error) {
      setPath("Error: " + error);
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
}, []);

useEffect(() => {
  if (path) {
    getForwardFiles();
  }
}, [path]);

  return (
    <ThemeProvider theme={theme}>
      <Box
        className="container"
        display={"flex"}
        height={"100vh"}
        width={"100vw"}
        bgcolor={"#282c34"}
      >
        <Typography color="#f6f6f6" variant="h6">Current path: {displayPath}</Typography>
        <form
          onSubmit={ (e) => {
            e.preventDefault()
            stepUp()
          }}
        >
          <Stack
            margin={2}
            spacing={2}
            direction="column"
            sx={{
              width: "15em"
            }}
          >
            <TextField
              required
              label="Path"
              text-color="#f6f6f6"
              onChange={
                (e) => {
                  setPath(e.target.value)
                  formatForOS(e.target.value)
                }
              }
            />
            <Button type="submit">
              Step Up
            </Button>
            {/* <Button type="submit">Submit</Button> */}
          </Stack>
        </form>
        {/* {dirsToPrint.length > 2 && (
          openFile(1)
        )} */}
        {dirsToPrint.length > 0 && (
          dirsToPrint.map((item, index) => (
            <Box
              className="mapBox"
              key={index}
              alignItems="center"
              border={1}
              borderRadius={3}
              borderColor={"#f6f6f6"}
              margin={1}
              justifyContent={"center"}
              sx={{
                width: "15em",
                height: "0.25em"
              }}
            >
              <Typography
                color="#f6f6f6"
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
