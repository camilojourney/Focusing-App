// Initializes the global Tauri helpers so other modules can safely call invoke/listen.
let tauriReadyResolve;
let tauriReady = false;

const tauriReadyPromise = new Promise((resolve) => {
  tauriReadyResolve = resolve;
});

function setupBridge() {
  console.log('window.__TAURI__ available?', typeof window.__TAURI__);
  console.log('window.__TAURI__ keys:', window.__TAURI__ ? Object.keys(window.__TAURI__) : 'undefined');

  if (!window.__TAURI__) {
    return false;
  }

  if (!window.Tauri) {
    window.Tauri = {};
  }

  try {
    window.Tauri.invoke = window.__TAURI__.core?.invoke || window.__TAURI__.tauri?.invoke;
    window.Tauri.listen = window.__TAURI__.event?.listen;
    window.Tauri.emit = window.__TAURI__.event?.emit;

    if (window.__TAURI__.webviewWindow) {
      const { WebviewWindow } = window.__TAURI__.webviewWindow;
      window.Tauri.appWindow = WebviewWindow.getCurrent();
    } else if (window.__TAURI__.window) {
      const { getCurrent } = window.__TAURI__.window;
      window.Tauri.appWindow = getCurrent();
    }

    tauriReady = true;
    tauriReadyResolve(window.Tauri);
    return true;
  } catch (error) {
    console.error('Error loading Tauri APIs:', error);
    return false;
  }
}

if (!setupBridge()) {
  window.addEventListener(
    'tauri://ready',
    () => {
      if (!tauriReady) {
        setupBridge();
      }
    },
    { once: true }
  );

  // Fallback so local browser previews don't hang forever.
  setTimeout(() => {
    if (!tauriReady) {
      console.warn('Tauri APIs not detected. Continuing without native bridge.');
      if (!window.Tauri) {
        window.Tauri = {};
      }
      tauriReadyResolve(window.Tauri);
    }
  }, 2000);
}

export function waitForTauriBridge() {
  return tauriReadyPromise;
}
