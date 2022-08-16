// const wasmPack = require("./wasm-pack.js");
// const watchman = require("./watchman.js");
// const localserver = require("./localserver.js");

// console.log("compiling wasm...");
// wasmPack.compileWasm();

// watchman.watch();
// localserver.startServer();

const {exec} = require('child_process');

// wasm-pack build C:\Users\Callum\Documents\git\rust-project --target web
exec("wasm-pack build --target web", (err, stdout, stderr) => {
    if (err) {
        console.log("could not execute command: ", err);
        return;
    }
    if (stdout) {
        console.log(stdout);
    }
    if (stderr) {
        console.log(stderr);
    }
});