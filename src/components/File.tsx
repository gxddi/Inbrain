import { useState } from "react";
import { ChevronRight, ChevronDown, Folder, FileText } from "lucide-react";
import { useFileContext } from "./FileContext";

export type FileNode = {
  name: string;
  path: string;
  is_folder: boolean;
  children: FileNode[] | null; // Accept other files as children or none
};

type FileProps = {
  node: FileNode;
  depth: number;
};

function File({ node, depth }: FileProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const { openFile } = useFileContext();

  const indent = depth * 16;

  function handleClick() {
    if (node.is_folder) {
      setIsExpanded(!isExpanded);
    } else {
      openFile(node.path);
    }
  }

  return (
    <>
      <div
        style={{ paddingLeft: `${indent}px` }}
        className="file-item"
        onClick={handleClick}
      >

        {node.is_folder ? (
          <>
            {isExpanded ? <ChevronDown /> : <ChevronRight />}
            <Folder />
          </>
        ) : (
          <>
            <FileText/>
          </>
        )}
        <span>{node.name}</span>

      </div>

      {node.is_folder && isExpanded && node.children?.map((child) => (
        <File key={child.name} node={child} depth={depth + 1} /> // For each folder/file within create folder/file
      ))}
    </>
  );
}

export default File;
