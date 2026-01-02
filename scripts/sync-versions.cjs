/* eslint-disable */
const fs = require('fs');
const path = require('path');

const version = process.argv[2];
const forPublish = process.argv[3] === '--publish';

if (!version) {
  console.error('Version required');
  process.exit(1);
}

// Update Cargo.toml
const cargoPath = path.join(__dirname, '..', 'Cargo.toml');
if (fs.existsSync(cargoPath)) {
  let cargoContent = fs.readFileSync(cargoPath, 'utf8');
  cargoContent = cargoContent.replace(
    /version = "[^"]+"/,
    `version = "${version}"`
  );
  fs.writeFileSync(cargoPath, cargoContent);
  console.log(`✓ Updated Cargo.toml to ${version}`);

  // Update Cargo.lock (only workspace packages, without updating external dependencies)
  const { execSync } = require('child_process');
  try {
    console.log('Updating Cargo.lock...');
    execSync('cargo metadata --format-version 1 > /dev/null 2>&1', {
      cwd: path.join(__dirname, '..'),
      shell: true
    });
    console.log(`✓ Updated Cargo.lock`);
  } catch (error) {
    console.error('⚠ Failed to update Cargo.lock:', error.message);
  }
}

// Update root package.json
const rootPackageJsonPath = path.join(__dirname, '..', 'package.json');
if (fs.existsSync(rootPackageJsonPath)) {
  const rootPackageJson = JSON.parse(fs.readFileSync(rootPackageJsonPath, 'utf8'));
  rootPackageJson.version = version;
  fs.writeFileSync(rootPackageJsonPath, JSON.stringify(rootPackageJson, null, 2) + '\n');
  console.log(`✓ Updated package.json to ${version}`);
}

// Update all package.json files
const packagesDir = path.join(__dirname, '..', 'packages');
if (fs.existsSync(packagesDir)) {
  const packages = fs.readdirSync(packagesDir);

  for (const pkg of packages) {
    const pkgJsonPath = path.join(packagesDir, pkg, 'package.json');
    if (fs.existsSync(pkgJsonPath)) {
      const pkgJson = JSON.parse(fs.readFileSync(pkgJsonPath, 'utf8'));
      pkgJson.version = version;

      // Update optionalDependencies versions for platform packages
      if (pkgJson.optionalDependencies) {
        for (const dep in pkgJson.optionalDependencies) {
          if (dep.startsWith('@archlinter/')) {
            // For publish: replace workspace: with actual version
            // For development: keep workspace:*
            if (forPublish) {
              pkgJson.optionalDependencies[dep] = version;
            } else if (!pkgJson.optionalDependencies[dep].startsWith('workspace:')) {
              pkgJson.optionalDependencies[dep] = 'workspace:*';
            }
          }
        }
      }

      // Update dependencies versions for internal packages
      if (pkgJson.dependencies) {
        for (const dep in pkgJson.dependencies) {
          if (dep.startsWith('@archlinter/')) {
            if (forPublish) {
              pkgJson.dependencies[dep] = version;
            } else if (!pkgJson.dependencies[dep].startsWith('workspace:')) {
              pkgJson.dependencies[dep] = 'workspace:*';
            }
          }
        }
      }

      fs.writeFileSync(pkgJsonPath, JSON.stringify(pkgJson, null, 2) + '\n');
      console.log(`✓ Updated packages/${pkg}/package.json to ${version}`);
    }
  }

  // Update pnpm-lock.yaml after changing package.json files
  if (!forPublish) {
    const { execSync } = require('child_process');
    try {
      execSync('pnpm install --lockfile-only', {
        cwd: path.join(__dirname, '..'),
        stdio: 'inherit'
      });
      console.log(`✓ Updated pnpm-lock.yaml`);
    } catch (error) {
      console.error('⚠ Failed to update pnpm-lock.yaml:', error.message);
    }
  }
}

console.log(`✓ Synced version ${version} to Cargo.toml and package.json files`);
