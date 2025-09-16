var Benchmark = require('benchmark');

async function benchmarkCell() {
  var suite = new Benchmark.Suite;

  const wasm = require('./pkg/externref_bench.js');

  suite.add('Bench', function() {
    wasm.bench();
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
