#!/usr/bin/env node
/* eslint-disable no-console */

const { platform, arch } = process;
const { existsSync } = require('node:fs');
const { join } = require('node:path');
const { spawnSync } = require('node:child_process');

function getBinaryName() {
  return platform === 'win32' ? 'archlint.exe' : 'archlint';
}

function isMusl() {
  if (platform !== 'linux') return false;
  try {
    const { execSync } = require('node:child_process');
    const output = execSync('ldd --version 2>&1 || true', { encoding: 'utf8' });
    return output.includes('musl');
  } catch {
    try {
      const os = require('node:fs').readFileSync('/etc/os-release', 'utf8');
      return os.includes('Alpine');
    } catch {
      return false;
    }
  }
}

function getPlatformKey() {
  const platformMap = {
    darwin: {
      arm64: 'darwin-arm64',
      x64: 'darwin-x64',
    },
    linux: {
      arm64: 'linux-arm64',
      x64: isMusl() ? 'linux-x64-musl' : 'linux-x64',
    },
    win32: {
      x64: 'win32-x64',
    },
  };

  const platformArch = platformMap[platform]?.[arch];
  if (!platformArch) {
    throw new Error(
      `Unsupported platform: ${platform}-${arch}. ` +
        `Supported: darwin-arm64, darwin-x64, linux-x64, linux-arm64, linux-x64-musl, win32-x64`
    );
  }

  return platformArch;
}

function getBinaryPath() {
  const platformKey = getPlatformKey();
  const binaryName = getBinaryName();

  const possiblePaths = [
    // 1. Local node_modules (npm/pnpm/yarn classic)
    join(__dirname, '..', '..', `cli-${platformKey}`, 'bin', binaryName),
    // 2. pnpm nested structure
    join(__dirname, '..', 'node_modules', '@archlinter', `cli-${platformKey}`, 'bin', binaryName),
    // 3. Development/Monorepo path
    join(__dirname, '..', '..', '..', 'packages', `cli-${platformKey}`, 'bin', binaryName),
  ];

  for (const p of possiblePaths) {
    if (existsSync(p)) {
      return p;
    }
  }

  // 4. Try require.resolve for package managers that support it
  try {
    const pkgName = `@archlinter/cli-${platformKey}`;
    // We try to resolve the package directory. Since main/bin might not be index.js,
    // we resolve the package.json path and go from there.
    const pkgJsonPath = require.resolve(`${pkgName}/package.json`);
    const pkgDir = join(pkgJsonPath, '..');
    const p = join(pkgDir, 'bin', binaryName);
    if (existsSync(p)) {
      return p;
    }
  } catch {
    // Package not installed or resolve failed
  }

  return null;
}

function run() {
  const binaryPath = getBinaryPath();

  if (!binaryPath) {
    const platformKey = getPlatformKey();
    console.error(`Error: Could not find archlint binary for ${platform}-${arch}`);
    console.error('');
    console.error('This usually means the optional dependency was not installed.');
    console.error('Try running: npm install @archlinter/cli --force');
    console.error('');
    console.error(`Or install the platform-specific package directly:`);
    console.error(`  npm install @archlinter/cli-${platformKey}`);
    process.exit(1);
  }

  const result = spawnSync(binaryPath, process.argv.slice(2), {
    stdio: 'inherit',
    env: process.env,
  });

  if (result.error) {
    if (result.error.code === 'EACCES') {
      console.error(`Error: Permission denied. Try: chmod +x "${binaryPath}"`);
    } else {
      console.error(`Error running archlint: ${result.error.message}`);
    }
    process.exit(1);
  }

  process.exit(result.status ?? 0);
}

run();
