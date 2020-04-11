var addon = require('../native');
const path = require('path');

// console.time();
// console.log(addon.hello(path.join(__dirname, '../img/big.jpeg'), path.join(__dirname, '../img/small.jpeg')));
// console.timeEnd();

console.time();
addon.image_search(path.join(__dirname, '../img/big.jpeg'),
  path.join(__dirname, '../img/small.jpeg'),
  (err, res) => {
    console.log(err, res);
    console.timeEnd();
  })
