const {exec} = require('child_process');

function compileWasm(done) {
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
        if(done) {
            done();
        }
    });
}

module.exports = {compileWasm};
