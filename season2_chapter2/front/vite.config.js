import { defineConfig } from 'vite';
import reactRefresh from '@vitejs/plugin-react-refresh';
import {VITE_SERVER_PORT, API_SERVER_PORT} from './src/lib/env.ts';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [reactRefresh()],
  server: {
    port: VITE_SERVER_PORT,
    proxy: {
      '/api': {
        target: `http://localhost:${API_SERVER_PORT}`,
        changeOrigin: true,
        logLevel: 'debug',
      },
    },
  },
  build: {
    outDir: 'dist/app'
  },
});
