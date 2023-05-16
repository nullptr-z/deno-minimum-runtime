function log(...args) {
  Deno.core.print("log: ");
  Deno.core.ops.log(...args);
}

function logs(...args) {
  Deno.core.print(...args);
}
