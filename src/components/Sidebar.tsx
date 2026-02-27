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
          <div className="sidebar-toolbar">
            <Menu className="sidebar-button" onClick={() => setIsOpen(false)} />
            <NewFile />
          </div>
          <FileTree />
        </aside>
      ) : (
        <Menu className="sidebar-button" onClick={() => setIsOpen(true)} />
      )}
    </>
  );
}

export default Sidebar;
