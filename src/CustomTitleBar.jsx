import React from "react";
import { getCurrent } from "@tauri-apps/api/window";
import "./titleBar.css";

function CustomTitleBar() {
  const currentWindow = getCurrent();

  const minimizeWindow = async () => {
    await currentWindow.minimize();
  };

  const maximizeWindow = async () => {
    if (await currentWindow.isMaximized()) {
      await currentWindow.unmaximize();
    } else {
      await currentWindow.maximize();
    }
  };

  const closeWindow = async () => {
    // You can change the default behavior here, e.g., hide the window instead of closing
    await currentWindow.hide();
  };

  return (
    <div className="title-bar" data-tauri-drag-region>
      <div className="title"></div>
      <div>
        {/* <button onClick={minimizeWindow}>Minimize</button>
        <button onClick={maximizeWindow}>Maximize/Unmaximize</button> */}
        <button onClick={closeWindow}>Close</button>
      </div>
    </div>
  );
}

export default CustomTitleBar;
