import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Box, Typography, TextField, Button, Stack } from "@mui/material";
import InsertDriveFileIcon from '@mui/icons-material/InsertDriveFile';
import FolderIcon from '@mui/icons-material/Folder';
import { ThemeProvider } from "@mui/material";
import theme from "./theme"
import "./App.css";

function App() {
  const [os, setOs] = useState("");
  const [path, setPath] = useState("");
  const [dirsToPrint, setDirsToPrint] = useState([]);
  const [displayPath, setDisplayPath] = useState("")

  let forwardDirsMap = new Map();

  //Windows support not currently working.
  async function formatForOS () {
    try {
      if (os === "windows") {
        setPath(await invoke("format_path_for_windows", { path: path }));
      }
    } catch (error) {
      setPath("Error: " + error);
    }
  }

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

  // Steps up one directory.
  async function stepUp() {
    try {
      let newPath = await invoke("step_up", { path: path });
      setDisplayPath(newPath);
      setPath(newPath);
    } catch (error) {
      setPath("Error: " + error);
    }
  }

  // Steps into a directory.
  async function stepIn(response) {
    try {
      response = "in 1";
      let _, pathResult = await invoke ("step_in", { response: response, pwd: path });
      setPath(pathResult);
    } catch (error) {
      setPath("Error: " + error)
    }
  }

  // Retrieves the operating system of the user running the program.
  // async function retrieveOs() {
    // try {
      // const result = await invoke("get_os");
      // setOs(result);
    // } catch (error) {
      // setOs("Error: " + error);
    // }
  // }

// When the path changes, get the forward files.
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
        alignItems={"center"}
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
          </Stack>
        </form>
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
      </Box>
    </ThemeProvider>
  );
}

export default App;
