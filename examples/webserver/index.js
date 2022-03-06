const net = require('node:net');
const { exec } = require('node:child_process');
const path = require('node:path');

//
//Change this constants to match your environment.
//

/** The server port */
const PORT = 3000;

/** The server host */
const HOST = '0.0.0.0';

/** The brainease binary location */
const BRAINEASE_BINARY = path.resolve(__dirname, '../../target/release/brain');

/** The brainease webserver code path */
const BRAINEASE_CODE = path.resolve(__dirname, 'main.brain');

//
//
//

net
  .createServer((socket) => {
    socket.on('data', (data) => {
      console.log(`Got request from ${socket.remoteAddress}:${socket.remotePort}`);

      // Escapes newline characters
      const httpString = data.toString().split('\r\n').join('\\n');
      const command = `echo "${httpString}" | ${BRAINEASE_BINARY} run ${BRAINEASE_CODE}`;

      // Calls brainease binary
      exec(command, { encoding: 'utf8', shell: 'bash' }, (_err, res, _stderr) => {
        socket.write(res.toString());

        // Disconnect after sending response
        socket.destroy();
      });
    });
  })
  .listen(PORT, HOST);
