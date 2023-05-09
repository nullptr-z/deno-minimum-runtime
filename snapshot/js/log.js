function log(...args) {
  Deno.core.ops.log(...args);
}
