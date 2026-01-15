import { IUserService } from '../domain/user.interface';

export interface UserServiceOptions {
    retryCount: number;
}

export class UserService implements IUserService {
    constructor(private options: UserServiceOptions) {}
    async findById(id: string) { return { id }; }
}
