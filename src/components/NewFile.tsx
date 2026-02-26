import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function NewFile() {
  const [showForm, setShowForm] = useState(false);
  const [fileName, setFileName] = useState("");

  async function handleCreate() {
    try {
      await invoke("create_file", { fileName });
    } catch (e) {
      console.log(e);
    }

    setShowForm(false);
    setFileName("");
  }

  return (
    <>
      <button onClick={() => setShowForm(true)}>
        New File
      </button>
      {showForm && (
        <form onSubmit={(e) => { e.preventDefault(); handleCreate(); }}>
          <input
            type="text"
            placeholder="Enter file name"
            value={fileName}
            onChange={(e) => setFileName(e.target.value)}
          />
          <button type="submit">Create</button>
          <button type="button" onClick={() => setShowForm(false)}>Cancel</button>
        </form>
      )}
    </>
  );
}

export default NewFile;
