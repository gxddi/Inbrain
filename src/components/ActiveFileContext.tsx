import React, { createContext, useContext, useState } from "react";
import { FileNode } from "./File"; // FileNode gives access to object created in ./File

type FileContextType = {
  activeFile: FileNode | null;
  setActiveFile: (file: FileNode) => void;
  activeFolder: FileNode | null;
  setActiveFolder: (folder: FileNode | null) => void;
  treeVersion: number;
  refreshTree: () => void;
};

const ActiveFileContext = createContext<FileContextType | null>(null)

export function ActiveFileContextProvider({ children }: { children: React.ReactNode }) {
  const [activeFile, setActiveFileState] = useState<FileNode | null>(null);
  const [activeFolder, setActiveFolder] = useState<FileNode | null>(null);
  const [treeVersion, setTreeVersion] = useState(0);

  function setActiveFile(file: FileNode) {
    setActiveFileState(file);
    setActiveFolder(file.parent);
  }

  function refreshTree() {
    setTreeVersion(v => v + 1);
  }

  return (
    <ActiveFileContext.Provider value={{ activeFile, setActiveFile, activeFolder, setActiveFolder, treeVersion, refreshTree }}>
      {children}
    </ActiveFileContext.Provider>
  );
}

export function useActiveFileContext() {
  return useContext(ActiveFileContext);
}
