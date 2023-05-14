
async function fetch_reqwest() {
  // const a = await
  return await fetch({ url: 'https://dummyjson.com/products/1', headers: { token: "abcd" }, method: 'gets' })
  // .then(res => Deno.core.print(123))
  // .then(json => Deno.core.print(json))
}

await fetch_reqwest()
