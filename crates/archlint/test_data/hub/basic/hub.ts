import { a } from './a';
import { b } from './b';
import { c } from './c';
import { d } from './d';
import { e } from './e';

export const hub = (x: any) => {
  return a(x) + b(x) + c(x) + d(x) + e(x);
};
