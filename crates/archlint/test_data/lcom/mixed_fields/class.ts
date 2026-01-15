export class MixedFields {
  private explicitField: number = 0;
  private anotherField: string = '';

  constructor(
    private readonly constructorField1: any,
    private readonly constructorField2: any,
  ) {}

  useExplicitField() {
    return this.explicitField;
  }

  useConstructorField1() {
    return this.constructorField1.get();
  }

  useConstructorField2() {
    return this.constructorField2.get();
  }

  useBothConstructorFields() {
    return this.constructorField1.get() + this.constructorField2.get();
  }

  useAllFields() {
    return this.explicitField + this.constructorField1.get() + this.constructorField2.get();
  }
}
