import { useState } from "react";
import "./App.css";
import SideBar from "./components/SideBar";
import CreateForm from "./components/CreateForm"

function App() {
  const [activeComponent, setActiveComponent] = useState(null);

  return (
    <div className="flex">

      <SideBar onIconClick={setActiveComponent} />
      {activeComponent === null && <CreateForm />}
      {activeComponent === 'CreateForm' && <CreateForm />}
      {activeComponent === 'ListComponent' && <ListComponent />}
      {activeComponent === 'ExportComponent' && <ExportComponent />}
    </div >
  );
}

export default App;
