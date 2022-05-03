import { build, BuildOptions } from 'esbuild';
import { define } from './define';

const buildOptions: BuildOptions = {
  entryPoints: ['src/index.tsx'],
  outdir: 'dist/build',
  loader: {
    '.svg': 'dataurl',
    '.md': 'text',
  },
  target: 'es2020',
  bundle: true,
  platform: 'browser',
  minify: true,
  define,
  logLevel: 'info',
};

build(buildOptions);

