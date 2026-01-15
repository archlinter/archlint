export interface IStorageProvider {
    save(id: string, data: any): Promise<void>;
}

export class S3StorageProvider implements IStorageProvider {
    async save(id: string, data: any) { /* implementation */ }
}
