const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const root = path.join(__dirname, '..');
const targetDir = path.join(root, 'target', 'release');

function getPlatformKey() {
  const { platform, arch } = process;

  function isMusl() {
    if (platform !== 'linux') return false;
    try {
      const output = execSync('ldd --version 2>&1 || true', { encoding: 'utf8' });
      return output.includes('musl');
    } catch {
      return false;
    }
  }

  const platformMap = {
    darwin: { arm64: 'darwin-arm64', x64: 'darwin-x64' },
    linux: { arm64: 'linux-arm64', x64: isMusl() ? 'linux-x64-musl' : 'linux-x64' },
    win32: { x64: 'win32-x64' },
  };

  return platformMap[platform]?.[arch];
}

function copy() {
  const platformKey = getPlatformKey();
  if (!platformKey) {
    console.error('Unsupported platform for local copy');
    return;
  }

  const binaryName = process.platform === 'win32' ? 'archlint.exe' : 'archlint';
  const source = path.join(targetDir, binaryName);
  const destDir = path.join(root, 'packages', `cli-${platformKey}`, 'bin');
  const dest = path.join(destDir, binaryName);

  if (!fs.existsSync(source)) {
    console.error(`Source binary not found: ${source}`);
    console.error('Run "cargo build --release" first.');
    process.exit(1);
  }

  if (!fs.existsSync(destDir)) {
    fs.mkdirSync(destDir, { recursive: true });
  }

  console.log(`Copying ${source} -> ${dest}`);
  fs.copyFileSync(source, dest);

  if (process.platform !== 'win32') {
    fs.chmodSync(dest, 0o755);
  }

  console.log('Done!');
}

copy();
