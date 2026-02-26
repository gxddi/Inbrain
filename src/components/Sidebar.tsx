import { useState } from "react";
import { Menu } from "lucide-react";
import NewFile from "./NewFile";
import FileTree from "./FileTree";
import "./Sidebar.css";

function Sidebar() {
  const [isOpen, setIsOpen] = useState(true);

  return (
    <>
      {isOpen ? (
        <aside className="sidebar">
          <Menu className="sidebar-button" onClick={() => setIsOpen(false)} />
          <NewFile />
          <FileTree />
        </aside>
      ) : (
        <Menu className="sidebar-button" onClick={() => setIsOpen(true)} />
      )}
    </>
  );
}

export default Sidebar;
