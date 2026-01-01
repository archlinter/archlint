export class Cohesive {
  private a = 1;

  method1() {
    return this.a;
  }
  method2() {
    return this.a;
  }
  method3() {
    return this.a;
  }
}
