const electron = require("electron");
const app = electron.app;
const browser_window = electron.BrowserWindow;

const path = require("path");
const is_dev = require("electron-is-dev");

let main_window;

function createWindow() {
 app.allowRendererProcessReuse = false;
 main_window = new browser_window({
  width: 1920 / 2,
  height: 1080 / 2,
  webPreferences: { preload: `${__dirname}/preload.js` }
 });
 main_window.loadURL(
  is_dev
   ? "http://localhost:3000"
   : `file://${path.join(__dirname, "../build/index.html")}`
 );
 main_window.on("closed", () => (main_window = null));
}

app.on("ready", createWindow);

app.on("window-all-closed", () => {
 if (process.platform !== "darwin") {
  app.quit();
 }
});

app.on("activate", () => {
 if (main_window === null) {
  createWindow();
 }
});
