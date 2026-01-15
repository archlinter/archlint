export class UsersService {
  private readonly logger: any;

  constructor(
    private readonly usersRepository: any,
    private readonly configService: any,
  ) {
    this.logger = console;
  }

  findOneById(id: string) {
    return this.usersRepository.findById(id);
  }

  findOneByIdOrThrow(id: string) {
    const user = this.findOneById(id);
    if (!user) {
      throw new Error('Not found');
    }
    return user;
  }

  findOneByAddress(address: string) {
    return this.usersRepository.findOne({ address });
  }

  update(id: string, data: any) {
    return this.usersRepository.update(id, data);
  }

  create(address: string) {
    return this.usersRepository.create({ address });
  }

  generateNonce(): string {
    return Math.random().toString();
  }
}
