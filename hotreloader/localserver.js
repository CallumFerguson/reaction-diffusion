// https://stackoverflow.com/questions/16333790/node-js-quick-file-server-static-files-over-http?page=1&tab=scoredesc#tab-top
var http = require('http');
var fs = require('fs');
var path = require('path');

function startServer() {
    http.createServer(function (request, response) {
        var filePath = path.join(__dirname, "..", request.url);
        if (path.extname(filePath) === "")
            filePath = path.join(filePath, "index.html");

        var contentType = 'text/html';
        switch (path.extname(filePath)) {
            case '.js':
                contentType = 'text/javascript';
                break;
            case '.css':
                contentType = 'text/css';
                break;
            case '.json':
                contentType = 'application/json';
                break;
            case '.png':
                contentType = 'image/png';
                break;
            case '.jpg':
                contentType = 'image/jpg';
                break;
            case '.wav':
                contentType = 'audio/wav';
                break;
            case '.wasm':
                contentType = 'application/wasm';
                break;
        }

        fs.readFile(filePath, function (error, content) {
            if (error) {
                if (error.code == 'ENOENT') {
                    fs.readFile('./404.html', function (error, content) {
                        if(error) {
                            content = "404";
                        }
                        response.writeHead(200, {'Content-Type': contentType});
                        response.end(content, 'utf-8');
                    });
                } else {
                    response.writeHead(500);
                    response.end('Sorry, check with the site admin for error: ' + error.code + ' ..\n');
                    response.end();
                }
            } else {
                response.writeHead(200, {'Content-Type': contentType});
                response.end(content, 'utf-8');
            }
        });

    }).listen(3000);
    console.log('Server running at http://127.0.0.1:3000');
}

module.exports = {startServer};