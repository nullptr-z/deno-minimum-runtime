Deno.core.print("package fetch\n\n");

async function fetch(url, method, headers, body) {
  return Deno.core.opAsync("fetch", { url, method, headers, body })
}
