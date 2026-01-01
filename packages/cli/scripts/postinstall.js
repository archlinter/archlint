// Postinstall script for @archlinter/cli
const { getPlatformKey } = require('./platform');

const platformKey = getPlatformKey();
if (platformKey) {
  console.log(`Successfully detected platform: ${platformKey}`);
} else {
  console.warn('Could not detect platform for @archlinter/cli');
}
