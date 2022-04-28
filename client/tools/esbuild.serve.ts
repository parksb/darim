import { build, BuildOptions, serve, ServeOptions } from 'esbuild';
import { define } from './define';
import { createServer, request } from 'http';
import { networkInterfaces } from 'os';

const port = 8000;
const servePort = 3000;

const serveOptions: ServeOptions = {
  servedir: 'public',
  port: servePort,
  onRequest: ({ remoteAddress, method, path, status, timeInMS }) => {
    console.log(`${remoteAddress} - "${method} ${path}" ${status} [${timeInMS}ms]`);
  },
};

const clients = [];
const buildOptions: BuildOptions = {
  entryPoints: ['src/index.tsx'],
  outdir: 'public/build',
  loader: {
    '.svg': 'dataurl',
    '.md': 'text',
  },
  target: 'es2020',
  bundle: true,
  platform: 'browser',
  define,
  logLevel: 'info',
  banner: { js: ' (() => new EventSource("/esbuild").onmessage = () => location.reload())();' },
  watch: {
    onRebuild(error) {
      clients.forEach((res) => res.write('data: update\n\n'));
      clients.length = 0;
      console.log(error ? 'failed to build' : '');
    },
  },
};

build(buildOptions).catch(() => process.exit(1));

serve(serveOptions, {}).then(() => {
  console.log(`> Local: http://127.0.0.1:${port}/`);
  console.log(`> Network: http://${networkInterfaces()['en0'][1].address}:${port}/\n`);

  createServer((req, res) => {
    const { url, method, headers } = req;

    if (req.url === '/esbuild') {
      return clients.push(
        res.writeHead(200, {
          'Content-Type': 'text/event-stream',
          'Cache-Control': 'no-cache',
          Connection: 'keep-alive',
        })
      );
    }

    const path = ~url.split('/').pop().indexOf('.') ? url : `/index.html`;
    req.pipe(
      request({ hostname: '0.0.0.0', port: servePort, path, method, headers }, (prxRes) => {
        res.writeHead(prxRes.statusCode, prxRes.headers);
        prxRes.pipe(res, { end: true });
      }),
      { end: true },
    );
  }).listen(port);
});
