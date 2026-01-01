// CLI platform detection helper
const { platform, arch } = process;

function isMusl() {
  if (platform !== 'linux') return false;
  const { execSync } = require('child_process');
  try {
    const output = execSync('ldd --version 2>&1 || true', { encoding: 'utf8' });
    return output.includes('musl');
  } catch {
    return false;
  }
}

function getPlatformKey() {
  if (platform === 'darwin' && arch === 'arm64') return 'darwin-arm64';
  if (platform === 'darwin' && arch === 'x64') return 'darwin-x64';
  if (platform === 'linux' && arch === 'x64') return isMusl() ? 'linux-x64-musl' : 'linux-x64';
  if (platform === 'linux' && arch === 'arm64') return 'linux-arm64';
  if (platform === 'win32' && arch === 'x64') return 'win32-x64';
  return null;
}

module.exports = { getPlatformKey };
