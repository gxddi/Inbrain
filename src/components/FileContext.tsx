import { createContext, useContext, useState } from "react";

type FileContextType = {
  openFilePath: string | null;
  openFile: (path: string) => void;
};

export const FileContext = createContext<FileContextType>({
  openFilePath: null,
  openFile: () => {},
});

export function FileProvider({ children }: { children: React.ReactNode }) {
  const [openFilePath, setOpenFilePath] = useState<string | null>(null);

  return (
    <FileContext.Provider value={{ openFilePath, openFile: setOpenFilePath }}>
      {children}
    </FileContext.Provider>
  );
}

export function useFileContext() {
  return useContext(FileContext);
}
