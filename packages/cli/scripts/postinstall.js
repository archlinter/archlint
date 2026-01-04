// Postinstall script for @archlinter/cli
/* eslint-disable no-console */
const { getPlatformKey } = require('./platform');

const platformKey = getPlatformKey();
if (platformKey) {
  console.log(`Successfully detected platform: ${platformKey}`);
} else {
  console.warn('Could not detect platform for @archlinter/cli');
}
