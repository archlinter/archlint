import { noCycles } from './no-cycles';
import { noGodModules } from './no-god-modules';
import { noDeadCode } from './no-dead-code';
import { noHighCoupling } from './no-high-coupling';
import { noBarrelAbuse } from './no-barrel-abuse';
import { noLayerViolations } from './no-layer-violations';
import { noSdpViolations } from './no-sdp-violations';
import { noHubModules } from './no-hub-modules';
import { noDeepNesting } from './no-deep-nesting';
import { noLongParams } from './no-long-params';
import { noHighComplexity } from './no-high-complexity';

export const rules = {
  'no-cycles': noCycles,
  'no-god-modules': noGodModules,
  'no-dead-code': noDeadCode,
  'no-high-coupling': noHighCoupling,
  'no-barrel-abuse': noBarrelAbuse,
  'no-layer-violations': noLayerViolations,
  'no-sdp-violations': noSdpViolations,
  'no-hub-modules': noHubModules,
  'no-deep-nesting': noDeepNesting,
  'no-long-params': noLongParams,
  'no-high-complexity': noHighComplexity,
};
