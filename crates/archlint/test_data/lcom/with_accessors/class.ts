export class DataService {
  private _value: number = 0;

  constructor(private readonly repository: any) {}

  get value(): number {
    return this._value;
  }

  set value(v: number) {
    this._value = v;
  }

  getData() {
    return this.repository.find(this._value);
  }

  setData(data: any) {
    return this.repository.save(data);
  }

  processData() {
    return this.repository.process(this._value);
  }
}
