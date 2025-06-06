import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Box, Typography, TextField, Button, Stack } from "@mui/material";
import InsertDriveFileIcon from '@mui/icons-material/InsertDriveFile';
import FolderIcon from '@mui/icons-material/Folder';
import { ThemeProvider } from "@mui/material";
import theme from "./theme"
import "./App.css";

function App() {
  const [path, setPath] = useState("");
  const [dirsToPrint, setDirsToPrint] = useState([]);
  const [displayPath, setDisplayPath] = useState("")
  const [selectedDir, setSelectedDir] = useState("");

  let forwardDirsMap = new Map();

  // Gets a list of the forward files and folders with a max depth of 1.
  async function getForwardFiles() {
    let boolean = true;
    try {
      let result = await invoke("output_files_as_vector", { path: path, printFiles: boolean});
      setDirsToPrint(result);
    } catch (error) {
      setDirsToPrint(["Error: ", error]);
    }
    finally {
      forwardDirsMap = dirsToPrint.map((item, index) => index);
      console.log(forwardDirsMap);
    }
  }

  // Gets a list of the forward files and folders with a max depth of 1.
  async function ls() {
    try {
      const result = await invoke("ls", { path: path });
      setDirsToPrint(result);
    } catch (error) {
      setDirsToPrint(["Error: ", error]);
    }
  }

  // Opens a file at the provided index in the current folder.
  async function openFile(index) {
    try {
      await invoke("open_file", { index: index, path: path });
    } catch (error) {
      setPath("Error: " + error);
    }
  }

  // Steps up one directory, setting the path to the new directory.
  async function stepUp() {
    try {
      let newPath = await invoke("step_up", { path: path });
      setDisplayPath(newPath);
      setPath(newPath);
    } catch (error) {
      setPath("Error: " + error);
    }
  }

  // Steps into a directory, taking a response from the user.
  async function stepIn(response) {
    try {
      // Must re-write this to use the selectedDir variable which contains a path
      // rather than a string with in and then the index of the folder.
      response = "in 1";
      let _, pathResult = await invoke ("step_in", { response: response, pwd: selectedDir });
      setPath(pathResult);
    } catch (error) {
      setPath("Error: " + error)
    }
  }

// When the path changes, get the forward files.
useEffect(() => {
  if (path) {
    getForwardFiles();
  }
  console.log(path);
}, [path]);

useEffect(() => {
  if (selectedDir) {
    console.log("selectedDir: ", selectedDir);
  }
}, [selectedDir]);

// When the path changes, get the forward files.
  return (
    <ThemeProvider theme={theme}>
      <Box
        className="container"
        display={"flex"}
        height={"100vh"}
        width={"100vw"}
        bgcolor={"#282c34"}
        alignItems={"center"}
      >
        <Box
          display={"flex"}
          justifyContent={"center"}
          alignItems={"center"}
          flexDirection={"column"}
          width={"100%"}
        >
          <Box
            position={"absolute"}
            display={"flex"}
            zIndex={1}
            width={"100%"}
            justifyContent={"center"}
            top={0}
          >
            <Typography color="#f6f6f6" variant="h6">File Explorer</Typography>
          </Box>
          <Box
            position={"absolute"}
            display={"flex"}
            zIndex={1}
            padding={2}
            width={"100%"}
            top={0}
            left={0}
          >
            <Typography color="#f6f6f6" variant="h4" fontSize={"1em"}>
              {
                path.split("\\").map((item, index) => {
                  if (index === 0) {
                    return item;
                  } else {
                    return " > " + item;
                  }
                })
              }
            </Typography>
          </Box>
        </Box>
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
            <Button 
              onClick={
                () => {
                  stepIn(selectedDir)
                }
              }
            >
              Step In
            </Button>
          </Stack>
        </form>
        <Stack
          margin={2}
          spacing={2}
          direction="column"
          justifyContent={"center"}
          alignItems={"center"}
          sx={{
            width: "45em",
            height: "100%",
            overflowY: "scroll",
            overflowX: "hidden",
            backgroundColor: "#282c34",
            borderRadius: "5px",
            padding: "10px",
          }}
        >
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
                onClick={
                  () => {
                    if (item.includes("Directory")) {
                      setSelectedDir(item);
                      stepIn(item)
                      console.log("Selected directory: ", item);
                    } else {
                      openFile(index);
                    }
                  }
                }
                sx={{
                  width: "30em",
                  height: "0.25em"
                }}
              >
                {item.includes("Directory") ? (
                  <FolderIcon
                    sx={{
                      marginRight: "0.5em"
                    }}
                  />
                ) : (
                  <InsertDriveFileIcon
                    sx={{
                      marginRight: "0.5em"
                    }}
                  />
                )}
                <Typography
                  color="#f6f6f6"
                  justifyContent="center"
                >
                  {item}
                </Typography>
              </Box>
            ))
          )}
        </Stack>
      </Box>
    </ThemeProvider>
  );
}

export default App;