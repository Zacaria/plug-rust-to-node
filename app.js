const path = require('path');
const ffi = require('ffi');

const hello = require('./lib');

// const lib = ffi.Library(path.join(__dirname, './native/target/debug/libplug_node'), {
//   multiply: ['int', ['int', 'int']],
//   concat: ['string', []],
// });
//
// const result = lib.multiply(15, 10);
// const result2 = lib.concat();

// console.log(result, result2);
console.log(hello);
