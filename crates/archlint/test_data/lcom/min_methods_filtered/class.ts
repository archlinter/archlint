export class ServiceWithConstructor {
  constructor(
    private readonly repo: any,
    private readonly config: any,
  ) {}

  method1() {
    return this.repo.find();
  }

  method2() {
    return this.repo.save();
  }
}
