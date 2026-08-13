#!/usr/bin/env node
'use strict';

/**
 * Fail if the Rust `tauri` crate minor and `@tauri-apps/api` minor diverge.
 * Portable: Node.js only (macOS and Windows GitHub Actions runners).
 *
 * Version sources, in order:
 *   Rust  — src-tauri/Cargo.lock package name "tauri"
 *   JS    — installed node_modules, else pnpm-lock.yaml, else an exact package.json pin
 */

const fs = require('fs');
const path = require('path');

const root = path.resolve(__dirname, '..');

function fail(message) {
  console.error(message);
  process.exit(1);
}

function readFile(relPath) {
  const abs = path.join(root, relPath);
  if (!fs.existsSync(abs)) {
    return null;
  }
  return fs.readFileSync(abs, 'utf8');
}

function minor(version) {
  const match = String(version).trim().match(/^(\d+)\.(\d+)/);
  if (!match) {
    fail(`Cannot parse semver minor from: ${version}`);
  }
  return `${match[1]}.${match[2]}`;
}

function rustTauriVersion() {
  const cargoLock = readFile(path.join('src-tauri', 'Cargo.lock'));
  if (!cargoLock) {
    fail('Missing src-tauri/Cargo.lock');
  }

  const blocks = cargoLock.split('[[package]]');
  for (const block of blocks) {
    const name = block.match(/^\s*name = "([^"]+)"/m);
    if (!name || name[1] !== 'tauri') {
      continue;
    }
    const version = block.match(/^\s*version = "([^"]+)"/m);
    if (version) {
      return version[1];
    }
  }
  fail('Could not find package name "tauri" in src-tauri/Cargo.lock');
}

function jsApiFromNodeModules() {
  const pkgPath = path.join(root, 'node_modules', '@tauri-apps', 'api', 'package.json');
  if (!fs.existsSync(pkgPath)) {
    return null;
  }
  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
  return pkg.version || null;
}

function stripLockVersion(raw) {
  return String(raw)
    .trim()
    .replace(/^['"]|['"]$/g, '')
    .split('(')[0]
    .split('_')[0]
    .trim();
}

function jsApiFromPnpmLock() {
  const text = readFile('pnpm-lock.yaml');
  if (!text) {
    return null;
  }

  const match = text.match(
    /(?:^|\n)[ \t]*['"]@tauri-apps\/api['"]:[ \t]*\r?\n(?:[ \t]+[^\n]+\r?\n)*?[ \t]+version:[ \t]*([^\s\n]+)/
  );
  if (!match) {
    return null;
  }
  return stripLockVersion(match[1]);
}

function jsApiFromPackageJson() {
  const text = readFile('package.json');
  if (!text) {
    return null;
  }
  const pkg = JSON.parse(text);
  const spec =
    (pkg.dependencies && pkg.dependencies['@tauri-apps/api']) ||
    (pkg.devDependencies && pkg.devDependencies['@tauri-apps/api']);
  if (!spec) {
    return null;
  }
  if (!/^\d+\.\d+\.\d+/.test(spec)) {
    return null;
  }
  return spec;
}

function jsApiVersion() {
  return (
    jsApiFromNodeModules() ||
    jsApiFromPnpmLock() ||
    jsApiFromPackageJson() ||
    fail(
      'Could not determine @tauri-apps/api version. Install dependencies or commit pnpm-lock.yaml.'
    )
  );
}

const rustVersion = rustTauriVersion();
const jsVersion = jsApiVersion();
const rustMinor = minor(rustVersion);
const jsMinor = minor(jsVersion);

console.log(`tauri crate ${rustVersion} (minor ${rustMinor})`);
console.log(`@tauri-apps/api ${jsVersion} (minor ${jsMinor})`);

if (rustMinor !== jsMinor) {
  fail(
    `Tauri JS/Rust minor mismatch: tauri crate ${rustVersion} vs @tauri-apps/api ${jsVersion}. ` +
      'Keep @tauri-apps/api on the same major.minor as the Rust tauri crate in src-tauri/Cargo.lock.'
  );
}

console.log('Tauri JS/Rust minors aligned.');
