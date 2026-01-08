export class Test {
  async get(id: string) {
    console.log('getting', id);
    const result = await this.repo.find(id);
    if (!result) return null;
    return result;
  }

  async get2(id: string) {
    console.log('getting', id);
    const result = await this.repo.find(id);
    if (!result) return null;
    return result;
  }
}
