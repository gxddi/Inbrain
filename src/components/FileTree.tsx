import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import File, { FileNode } from "./File"; // FileNode gives access to object created in ./File
import { useActiveFileContext } from "./ActiveFileContext";
import "./FileTree.css";

function FileTree() {
  const [tree, setTree] = useState<FileNode | null>(null);
  const [error, setError] = useState<string | null>(null);
  const { treeVersion } = useActiveFileContext()!;

  useEffect(() => {
    invoke<FileNode>("get_root_node")
      .then(setTree)
      .catch((e) => setError(e));
  }, [treeVersion]);

  if (error) return <p>{error}</p>;
  if (!tree) return <p>Loading...</p>;

  return (
    <div className="file-tree">
      {tree.children?.map((node) => ( // If there's anything in tree.children...
        <File key={node.name} node={node} depth={0} />
      ))}
    </div>
  );
}

export default FileTree;
