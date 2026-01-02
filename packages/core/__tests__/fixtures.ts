import { join } from 'path';

export const TEST_DATA = join(__dirname, '..', '..', '..', 'crates', 'archlint', 'test_data');

export const fixtures = {
  cycles: join(TEST_DATA, 'cycles'),
  godModule: join(TEST_DATA, 'god_module'),
  deadCode: join(TEST_DATA, 'dead_code'),
};
