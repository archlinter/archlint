export function archlintHelp(): { content: { type: 'text'; text: string }[] } {
  const helpText = `
# archlint MCP Server Usage Guide

This server allows you to analyze TypeScript/JavaScript project architecture and detect "smells" (architectural issues).

## Available Tools

### 1. archlint_scan
Performs a full architectural scan of the project.
- **path**: (required) Absolute path to the project.
- **force**: (optional) Set to true to ignore caches and re-scan.

### 2. archlint_get_stats
Get a summary of architectural health (grade, counts of issues).
- **path**: (required) Absolute path to the project.

### 3. archlint_get_smells
Get detailed information about detected smells with filtering and pagination.
- **path**: (required) Absolute path to the project.
- **types**: (optional) Filter by smell type (e.g., ["CyclicDependency", "GodModule"]).
- **severity**: (optional) Minimum severity (low, medium, high, critical).
- **file**: (optional) Filter smells by file path.
- **minScore**: (optional) Filter smells by minimum score.
- **limit/offset**: (optional) For pagination.

### 4. archlint_list_detectors
List all available detectors with their descriptions.

### 5. archlint_clear_cache
Clear the MCP memory cache or full cache.

## Workflow Tips
- Use \`archlint_scan\` first to get an overview of a project.
- Use \`archlint_get_smells\` to deep dive into specific issue types or high-severity problems.
- Always provide the absolute path to the project root.
`;

  return {
    content: [
      {
        type: 'text',
        text: helpText,
      },
    ],
  };
}
