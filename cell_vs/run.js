var Benchmark = require('benchmark');

async function benchmarkCell() {
  var suite = new Benchmark.Suite;

  const fs = require('node:fs');

  const cellBuffer = fs.readFileSync('target/cell/wasm32-unknown-unknown/release/cell_vs.wasm');
  const cell  = await WebAssembly.instantiate(cellBuffer);

  const refCellBuffer = fs.readFileSync('target/ref_cell/wasm32-unknown-unknown/release/cell_vs.wasm');
  const refCell = await WebAssembly.instantiate(refCellBuffer);

  const unsafeCellBuffer = fs.readFileSync('target/unsafe_cell/wasm32-unknown-unknown/release/cell_vs.wasm');
  const unsafeCell = await WebAssembly.instantiate(unsafeCellBuffer);

  suite.add('Cell', function() {
    cell.instance.exports.set(0);
  })
  .add('RefCell', function() {
    refCell.instance.exports.set(0);
  })
  .add('UnsafeCell', function() {
    unsafeCell.instance.exports.set(0);
  })
  .on('cycle', function(event) {
    console.log(String(event.target));
  })
  .on('complete', function() {
    console.log('Fastest is ' + this.filter('fastest').map('name'));
  })
  .run({ 'async': true });
}

benchmarkCell();
