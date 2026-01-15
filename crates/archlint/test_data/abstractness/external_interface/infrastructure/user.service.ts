import { IUserService, User } from '../domain/user.interface';

export class UserService implements IUserService {
    private users: User[] = [];

    async findById(id: string): Promise<User | null> {
        return this.users.find(u => u.id === id) || null;
    }

    async create(data: Partial<User>): Promise<User> {
        const user = { id: '1', email: 'test@example.com', ...data } as User;
        this.users.push(user);
        return user;
    }
}
