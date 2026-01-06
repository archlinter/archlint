/* eslint-disable */
const { execSync } = require('child_process');

const version = process.env.npm_package_version || process.argv[2];
const message = process.argv[3] || `chore(release): ${version}`;

if (!version) {
  console.error('Version required');
  process.exit(1);
}

try {
  // Reset any workflow files from staging area
  execSync('git reset HEAD .github/workflows/ 2>/dev/null || true', {
    stdio: 'inherit',
    shell: true
  });

  // Add only the files we want to commit
  const files = [
    'CHANGELOG.md',
    'Cargo.toml',
    'Cargo.lock',
    'packages/*/package.json',
    'package.json',
    'pnpm-lock.yaml'
  ];

  // Add files one by one to avoid issues with globs
  for (const file of files) {
    try {
      execSync(`git add ${file}`, {
        stdio: 'inherit',
        shell: true
      });
    } catch (error) {
      // Ignore errors for files that don't exist or aren't changed
      console.warn(`Warning: Could not add ${file}`);
    }
  }

  // Reset workflow files again in case they were added
  execSync('git reset HEAD .github/workflows/ 2>/dev/null || true', {
    stdio: 'inherit',
    shell: true
  });

  // Check if there are any staged changes
  const status = execSync('git diff --cached --name-only', {
    encoding: 'utf8',
    shell: true
  }).trim();

  if (!status) {
    console.log('No changes to commit');
    process.exit(0);
  }

  // Commit the changes
  execSync(`git commit -m "${message}" --no-verify`, {
    stdio: 'inherit',
    shell: true
  });

  console.log(`✓ Committed release changes for version ${version}`);
} catch (error) {
  console.error('Error committing release changes:', error.message);
  process.exit(1);
}

// Note: semantic-release will handle the push automatically after prepare steps

