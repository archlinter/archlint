import { IUserService } from '../domain/user.interface';
import { UserServiceOptions } from '../infrastructure/user.service';

export class UserController10 {
    // Depends on domain interface (no edge to user.service.ts)
    // AND depends on UserServiceOptions (abstract edge to user.service.ts)
    constructor(
        private readonly userService: IUserService,
        private readonly options: UserServiceOptions 
    ) {}
}
