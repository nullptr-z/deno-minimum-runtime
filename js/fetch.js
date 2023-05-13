
(async function fetch_reqwest() {
  const a = await fetch('https://dummyjson.com/products/1')
    .then(res => Deno.core.print(`123`))
    .then(json => Deno.core.print(json))

  Deno.core.print(`123${a}`)

})()
