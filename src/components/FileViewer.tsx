import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useActiveFileContext } from "./ActiveFileContext";
import "./FileViewer.css";

function FileViewer() {
  const { activeFile } = useActiveFileContext()!;
  const [content, setContent] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!activeFile) return;

    setContent(null);
    setError(null);

    invoke<string>("read_file", { node: activeFile })
      .then(setContent)
      .catch((e) => setError(e));
  }, [activeFile]);

  return (
    <div className="file-viewer">
      {!activeFile && <p className="placeholder">Select a file to view</p>}
      {error && <p className="error">{error}</p>}
      {content !== null && <pre>{content}</pre>}
    </div>
  );
}

export default FileViewer;
