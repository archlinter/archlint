#!/usr/bin/env node
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { createServer } from './server.js';

async function main() {
  const server = createServer();
  const transport = new StdioServerTransport();

  await server.connect(transport);
  // eslint-disable-next-line no-console
  console.error('archlint MCP server running on stdio');
}

main().catch((error) => {
  // eslint-disable-next-line no-console
  console.error('Fatal error in MCP server:', error);
  process.exit(1);
});
