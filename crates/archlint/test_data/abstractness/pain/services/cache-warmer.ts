import { DatabaseService } from '../database.service';
export class CacheWarmer {
    constructor(private db: DatabaseService) {}
}
