import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useFileContext } from "./FileContext";
import "./FileViewer.css";

function FileViewer() {
  const { openFilePath } = useFileContext();
  const [content, setContent] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!openFilePath) return;

    setContent(null);
    setError(null);

    invoke<string>("read_file", { fileName: openFilePath })
      .then(setContent)
      .catch((e) => setError(e));
  }, [openFilePath]);

  return (
    <div className="file-viewer">
      {!openFilePath && <p className="placeholder">Select a file to view</p>}
      {error && <p className="error">{error}</p>}
      {content !== null && <pre>{content}</pre>}
    </div>
  );
}

export default FileViewer;
