var addon = require('..');
const path = require('path');

console.time('image_search');
addon.image_search(path.join(__dirname, './img/big.png'),
  path.join(__dirname, './img/small.png'),
  {
    out: path.join(__dirname, './temp.jpeg'),
    result_level: 2,
  },
  (err, res) => {
    console.log('image_search', err, res);
    console.timeEnd('image_search');
  });

console.time('get_d_hash');
addon.get_d_hash(path.join(__dirname, './img/small.png'),
  (err, res) => {
    console.log('get_d_hash', err, res);
    console.timeEnd('get_d_hash');
  });

console.time('get_hamming_distance_by_hex_hash');
let hamming_distance = addon.get_hamming_distance_by_hex_hash('3731316430182B65', '3631314430105A64');
console.log('hamming_distance', hamming_distance);
console.timeEnd('get_hamming_distance_by_hex_hash');

console.log('Main thread task...');