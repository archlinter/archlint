export class MethodCalls {
  private field: any;

  constructor(private readonly repo: any) {}

  method1() {
    return this.repo.find();
  }

  method2() {
    return this.method1();
  }

  method3() {
    return this.method2();
  }

  method4() {
    return this.field;
  }
}
