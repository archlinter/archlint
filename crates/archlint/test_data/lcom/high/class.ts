export class LowCohesion {
  private a = 1;
  private b = 2;
  private c = 3;

  method1() {
    return this.a;
  }
  method2() {
    return this.b;
  }
  method3() {
    return this.c;
  }
}
