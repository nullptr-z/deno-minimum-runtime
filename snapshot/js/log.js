function log(...args) {
  Deno.core.print("log: ");
  Deno.core.ops.log(...args);
}
