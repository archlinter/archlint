import { dep1 } from './dep1';
import { dep2 } from './dep2';
export const god = () => dep1() + dep2();
