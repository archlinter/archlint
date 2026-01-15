export interface User {
    id: string;
    email: string;
}

export interface IUserService {
    findById(id: string): Promise<User | null>;
    create(data: Partial<User>): Promise<User>;
}
