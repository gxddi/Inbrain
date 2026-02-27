import "./App.css";
import Sidebar from "./components/Sidebar";
import FileViewer from "./components/FileViewer";
import { FileProvider } from "./components/FileContext";

function App() {
  return (
    <div className="container">
      <div className="side">
        <Sidebar />
      </div>
      <div className="main">
        <FileViewer/>
      </div>
    </div>
  );
}

export default App;
