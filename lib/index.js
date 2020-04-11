var addon = require('../native');
const path = require('path');

console.time();
addon.image_search(path.join(__dirname, '../img/big.jpeg'),
  path.join(__dirname, '../img/small.jpeg'),
  (err, res) => {
    console.log(err, res);
    console.timeEnd();
  })

console.log('Main thread task...');
