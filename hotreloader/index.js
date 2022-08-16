import {WebSocket, WebSocketServer} from 'ws';
import compileWasm from "./compile-wasm.js";
import watch from "./watchman.js";
import startServer from "./localserver.js";

const wss = new WebSocketServer({port: 3001});
const connections = [];
wss.on('connection', function connection(ws) {
    connections.push(ws);

    ws.on("error", () => {
        const i = connections.indexOf(ws);
        if (i !== -1) {
            connections.splice(i, 1);
        }
    })

    ws.on("close", () => {
        const i = connections.indexOf(ws);
        if (i !== -1) {
            connections.splice(i, 1);
        }
    })
});

console.log("compiling wasm...");
compileWasm(() => {
    watch(() => {
        connections.forEach((connection) => {
            if (connection && connection.readyState !== WebSocket.CLOSED && connection.readyState !== WebSocket.CLOSING) {
                connection.send("change");
            }
        })
    });
    startServer();
});
