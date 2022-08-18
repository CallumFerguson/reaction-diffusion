import {exec} from "child_process";

export default function compileWasm(done) {
    exec("wasm-pack build --target web", (err, stdout, stderr) => {
        if (err) {
            console.log("could not execute command: ", err);
            done();
            return;
        }
        if (stdout) {
            console.log(stdout);
        }
        if (stderr) {
            console.log(stderr);
        }
        if (done) {
            done();
        }
    });
}

