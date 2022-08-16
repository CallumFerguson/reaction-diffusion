const {exec} = require('child_process');

var watchman = require('fb-watchman');
var client = new watchman.Client();

var dir_of_interest = "C:\\Users\\Callum\\CLionProjects\\untitled1\\src";

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

                make_subscription(client, resp.watch, resp.relative_path);
            });
    });

// `watch` is obtained from `resp.watch` in the `watch-project` response.
// `relative_path` is obtained from `resp.relative_path` in the
// `watch-project` response.
function make_subscription(client, watch, relative_path) {
    sub = {
        // Match any `.rs` file in the dir_of_interest
        expression: ["allof", ["match", "*.rs"]],
        // Which fields we're interested in
        fields: ["name", "size", "mtime_ms", "exists", "type"]
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

    // Subscription results are emitted via the subscription event.
    // Note that this emits for all subscriptions.  If you have
    // subscriptions with different `fields` you will need to check
    // the subscription name and handle the differing data accordingly.
    // `resp`  looks like this in practice:
    //
    // { root: '/private/tmp/foo',
    //   subscription: 'mysubscription',
    //   files: [ { name: 'node_modules/fb-watchman/index.js',
    //       size: 4768,
    //       exists: true,
    //       type: 'f' } ] }
    client.on('subscription', function (resp) {
        if (resp.subscription !== 'mysubscription') return;

        console.log("compiling wasm...");

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

        // resp.files.forEach(function (file) {
        //     // convert Int64 instance to javascript integer
        //     const mtime_ms = +file.mtime_ms;
        //
        //     // console.log('file changed: ' + file.name, mtime_ms);
        //
        //
        //     // wasm-pack build --target web
        //     exec("echo test", (err, stdout, stderr) => {
        //         if (err) {
        //             console.log("could not execute command: ", err);
        //             return;
        //         }
        //         if (stdout) {
        //             console.log(stdout);
        //         }
        //         if (stderr) {
        //             console.log(stderr);
        //         }
        //     });
        // });
    });
}