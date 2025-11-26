import { defineConfig } from 'vite';

export default defineConfig(() => ({
  root: 'src',
  base: './',
  clearScreen: false,
  envPrefix: ['VITE_', 'TAURI_'],
  server: {
    host: '127.0.0.1',
    port: 1420,
    strictPort: true,
    hmr: {
      protocol: 'ws',
      host: '127.0.0.1',
      port: 1421
    }
  },
  build: {
    outDir: '../dist',
    emptyOutDir: true,
    target: ['es2021', 'chrome100', 'safari13'],
    rollupOptions: {
      input: {
        main: 'src/index.html',
        settings: 'src/settings.html'
      }
    }
  }
}));
