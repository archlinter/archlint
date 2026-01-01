import { a } from './a';
import { b } from './b';
import { c } from './c';
import { d } from './d';
import { e } from './e';

export const god = (x: any) => {
  if (x > 0) {
    if (x > 10) {
      if (x > 20) {
        return a(x);
      }
      return b(x);
    } else {
      if (x > 5) {
        return c(x);
      }
      return d(x);
    }
  } else {
    switch (x) {
      case -1:
        return c(x);
      case -2:
        return d(x);
      case -3:
        return e(x);
      case -4:
        return a(x);
      default:
        return e(x);
    }
  }
};
