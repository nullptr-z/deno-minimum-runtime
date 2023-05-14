Deno.core.print("package fetch\n\n");

((window) => {

  async function fetch(params) {
    if (typeof params === 'string') {
      return await Deno.core.opAsync("fetch", { url: params, method: 'get' })
    } else if (typeof params === 'object') {
      return await Deno.core.opAsync("fetch", params)
    } else {
      throw Error("the request params type is error")
    }
  }

  window.fetch = fetch
})(this)
