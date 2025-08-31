import ChatComponent from "./components/ChatComponent";
import React from "react";
import "./App.css";

const App: React.FC = () => {
  return (
    <main className="app-container">
      <div className="playground-card">
        <h1 className="title">LLM Playground</h1>
        <ChatComponent />
      </div>
    </main>
  )
};

export default App;
