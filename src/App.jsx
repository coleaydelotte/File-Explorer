import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [name, setName] = useState("");
  const [arr, setArr] = useState([]);
  const [dirsToPrint, setDirsToPrint] = useState([]);

  // async function getArr() {
  //   setArr(await invoke("get_arr", { name: "Tauri" }));
  // }
  // getArr();

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


  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // Have not gotten rid of base front end because I want to learn the hover effect.
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <p>Hello World!:</p>
      {/* {arr.map((item, index) => (
        <p key={index}>{item}</p>
      ))} */}
      <p>Files:</p>
      {dirsToPrint.map((item, index) => (
        <p key={index}>{item}</p>
      ))}
    </div>
  );
}

export default App;
