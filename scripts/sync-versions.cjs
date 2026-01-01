const fs = require('fs');
const path = require('path');

const version = process.argv[2];
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
            // Keep workspace:* or other workspace: protocols as is
            if (!pkgJson.optionalDependencies[dep].startsWith('workspace:')) {
              pkgJson.optionalDependencies[dep] = version;
            }
          }
        }
      }

      fs.writeFileSync(pkgJsonPath, JSON.stringify(pkgJson, null, 2) + '\n');
      console.log(`✓ Updated packages/${pkg}/package.json to ${version}`);
    }
  }
}

console.log(`✓ Synced version ${version} to Cargo.toml and package.json files`);
