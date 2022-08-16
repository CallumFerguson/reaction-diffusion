const wasmPack = require("./compile-wasm.js");
const watchman = require("./watchman.js");
const localserver = require("./localserver.js");

console.log("compiling wasm...");
wasmPack.compileWasm();

// watchman.watch();
// localserver.startServer();