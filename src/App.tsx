import "./App.css";
import Sidebar from "./components/Sidebar";
import FileViewer from "./components/FileViewer";
import { ActiveFileContextProvider } from "./components/ActiveFileContext";

function App() {
  return (
    <ActiveFileContextProvider>
      <main className="container">
        <div className="side">
          <Sidebar />
        </div>
        <div className="viewer">
          <FileViewer/>
        </div>
      </main>
    </ActiveFileContextProvider>  
  );
}

export default App;
