
async function fetch_reqwest() {
  // return await fetch({
  //   url: 'https://dummyjson.com/products/add',
  //   method: 'POST',
  //   headers: { 'Content-Type': 'application/json' },
  //   query: { id: 102 },
  //   body: {
  //     id: 102,
  //     title: 'BMW Pencils',
  //   }
  // })

  return await fetch({ url: 'https://dummyjson.com/products/search', query: 'q=phone 9&price=549' })
}

const res = await fetch_reqwest()
Deno.core.print(`status: ${res.status} \n`)
log(JSON.stringify(res.json(), null, 2))
