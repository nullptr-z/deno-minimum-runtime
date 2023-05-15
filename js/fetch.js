
async function fetch_reqwest() {
  // return await fetch({
  //   url: 'https://dummyjson.com/products/add?',
  //   method: 'POST',
  //   headers: { 'Content-Type': 'application/json' },
  //   query: { a: 2, b: 3 },
  //   body: JSON.stringify({
  //     title: 'BMW Pencil',
  //   })
  // })

  return await fetch({ url: 'https://dummyjson.com/products/search', query: "q=phone&b=123" })
}

const res = await fetch_reqwest()
Deno.core.print(JSON.stringify(res, null, 2))
