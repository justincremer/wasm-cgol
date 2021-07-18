const rust = import("./dist/wasm_cgol");

rust.then((fn) => {
  fn._createElement();
  fn._alert("This is from javascript");
});
