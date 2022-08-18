// https://facebook.github.io/watchman/docs/nodejs.html

import compileWasm from "./compile-wasm.js";

import * as watchman from "fb-watchman";

var client = new watchman.Client();

var dir_of_interest = "C:\\Users\\Callum\\Documents\\git\\rust-project\\src";

export default function watch(change) {
    client.capabilityCheck({optional: [], required: ['relative_root']},
        function (error, resp) {
            if (error) {
                console.log(error);
                client.end();
                return;
            }

            // Initiate the watch
            client.command(['watch-project', dir_of_interest],
                function (error, resp) {
                    if (error) {
                        console.error('Error initiating watch:', error);
                        return;
                    }

                    // It is considered to be best practice to show any 'warning' or
                    // 'error' information to the user, as it may suggest steps
                    // for remediation
                    if ('warning' in resp) {
                        console.log('warning: ', resp.warning);
                    }

                    // `watch-project` can consolidate the watch for your
                    // dir_of_interest with another watch at a higher level in the
                    // tree, so it is very important to record the `relative_path`
                    // returned in resp

                    console.log('watching for changes to *.rs in ', resp.watch,
                        ' relative_path', resp.relative_path);
                    make_time_constrained_subscription(client, resp.watch, resp.relative_path, change);
                });
        });
}

function make_time_constrained_subscription(client, watch, relative_path, change) {
    client.command(['clock', watch], function (error, resp) {
        if (error) {
            console.error('Failed to query clock:', error);
            return;
        }

        const sub = {
            // Match any `.js` file in the dir_of_interest
            expression: ["allof", ["match", "*.rs"]],
            // Which fields we're interested in
            fields: ["name", "size", "exists", "type"],
            // add our time constraint
            since: resp.clock
        };

        if (relative_path) {
            sub.relative_root = relative_path;
        }

        client.command(['subscribe', watch, 'mysubscription', sub],
            function (error, resp) {
                if (error) {
                    // Probably an error in the subscription criteria
                    console.error('failed to subscribe: ', error);
                    return;
                }
                // console.log('subscription ' + resp.subscribe + ' established');
            });

        let compileInProgress = false;
        let skippedCompile = false;
        let lastResp = null;

        let subscription = (resp) => {
            lastResp = resp;
            if (resp.subscription !== 'mysubscription') return;
            if (resp.files.length === 0) return;

            if (compileInProgress) {
                console.log("wasm compile in progress. waiting for it to finish before compiling...");
                skippedCompile = true;
                return;
            }
            console.log("compiling wasm...");
            compileInProgress = true;
            compileWasm(() => {
                compileInProgress = false;
                if (change) {
                    change();
                }
                if(skippedCompile) {
                    skippedCompile = false;
                    subscription(lastResp);
                }
            });
        };

        client.on('subscription', subscription);
    });
}
