Deno.core.print("package fetch\n\n");

((window) => {

  async function fetch(args) {
    const argsType = typeof args;
    if (argsType === 'string') {
      args = { url: args, method: 'get' }
    } else if (argsType === 'object') {
      if (!args.url) throw Error("the request url is empty")
    } else {
      throw Error("the request args type is error")
    }

    const res = await Deno.core.opAsync("op_fetch", args)

    res.text = () => {
      const body = res.body
      if (!body) return null
      return body
      return Deno.core.opAsync("op_decode_utf8", body)
      Deno.core.print(`${a}`);
    }

    res.json = () => {
      const text = res.text()
      if (!text) return null

      return JSON.parse(text)
    }

    return res
  }

  window.fetch = fetch
})(this)
