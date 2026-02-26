import "./App.css";
import Sidebar from "./components/Sidebar";
import FileViewer from "./components/FileViewer";
import { FileProvider } from "./components/FileContext";

function App() {
  return (
    <FileProvider>
      <main className="container">
        <Sidebar />
        <FileViewer />
      </main>
    </FileProvider>
  );
}

export default App;
