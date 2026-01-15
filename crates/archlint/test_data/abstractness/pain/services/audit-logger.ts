import { DatabaseService } from '../database.service';
export class AuditLogger {
    constructor(private db: DatabaseService) {}
}
