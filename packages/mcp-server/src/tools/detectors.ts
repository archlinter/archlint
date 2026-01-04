import { getDetectors } from '@archlinter/core';
import { ListDetectorsInput } from '../schemas.js';
import { formatResult, formatDetectorsMd } from '../formatters.js';

export function archlintListDetectors(
  input: ListDetectorsInput
): { content: { type: 'text'; text: string }[] } {
  const { format } = input;
  const detectors = getDetectors();

  return {
    content: [
      {
        type: 'text',
        text: formatResult(detectors, format, formatDetectorsMd),
      },
    ],
  };
}
