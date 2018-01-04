const path = require('path');
const ffi = require('ffi');

const lib = ffi.Library(path.join(__dirname, './target/debug/libplug_node'), {
  multiply: ['int', ['int', 'int']],
});

function multiply(a, b) {
  return a * b;
}

const result = lib.multiply(15, 10);

console.log(result);
