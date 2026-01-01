import { db } from '../infra/db';
export const getUser = () => db.find();
