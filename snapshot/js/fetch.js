Deno.core.print("package fetch\n\n");

async function fetch(url, method = "GET", headers, body) {
  Deno.core.opAsync("fetch", { url, method, headers, body })
}
