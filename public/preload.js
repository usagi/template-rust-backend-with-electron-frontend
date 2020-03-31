try {
 // Export to an electron client ( App.js and etc. )
 window.preload = {
  is_dev: require("electron-is-dev"),
  native: require("../native")

  // Note: Uncomment if you wanto use `electron.remote` in App.js or elsewhere
  // , remote: require( 'electron' ).remote;
 };
} catch (e) {
 const fs = require("fs");
 fs.writeFileSync("preload.error.log", e);
}
