import { UserService } from '../infrastructure/user.service';
import { User } from '../domain/user.interface';

export class UserController2 {
    constructor(private readonly userService: UserService) {}

    async getUser(id: string): Promise<User | null> {
        return this.userService.findById(id);
    }
}
