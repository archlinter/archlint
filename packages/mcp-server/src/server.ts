import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';
import { SERVER_NAME, SERVER_VERSION } from './constants.js';
import {
  ScanInputSchema,
  GetSmellsInputSchema,
  GetStatsInputSchema,
  ListDetectorsInputSchema,
  ClearCacheInputSchema,
} from './schemas.js';
import { archlintScan } from './tools/scan.js';
import { archlintGetSmells } from './tools/smells.js';
import { archlintGetStats } from './tools/stats.js';
import { archlintListDetectors } from './tools/detectors.js';
import { archlintClearCache } from './tools/cache_tool.js';
import { archlintHelp } from './tools/help.js';

export function createServer() {
  const server = new McpServer({
    name: SERVER_NAME,
    version: SERVER_VERSION,
  });

  // Register Tools
  server.tool(
    'archlint_scan',
    'Perform a full architectural scan of a project',
    ScanInputSchema.shape,
    archlintScan
  );

  server.tool(
    'archlint_get_smells',
    'Get detailed architectural smells with filtering and pagination',
    GetSmellsInputSchema.shape,
    archlintGetSmells
  );

  server.tool(
    'archlint_get_stats',
    'Get project architectural statistics and grade',
    GetStatsInputSchema.shape,
    archlintGetStats
  );

  server.tool(
    'archlint_list_detectors',
    'List all available archlint detectors',
    ListDetectorsInputSchema.shape,
    archlintListDetectors
  );

  server.tool(
    'archlint_clear_cache',
    'Clear archlint caches',
    ClearCacheInputSchema.shape,
    archlintClearCache
  );

  server.tool('archlint_help', 'Get help and usage guide for archlint MCP tools', {}, archlintHelp);

  return server;
}
