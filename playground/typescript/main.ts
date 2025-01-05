import { map, filter, reduce } from 'lodash';
const numbers = [1, 2, 3, 4];

const doubled = map(numbers, (num) => num * 2); // [2, 4, 6, 8]
