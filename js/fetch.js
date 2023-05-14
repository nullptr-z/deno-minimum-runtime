
async function fetch_reqwest() {
  return await fetch({
    url: 'https://dummyjson.com/products/add&a=1&b=2',
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      title: 'BMW Pencil',
    })
  })
}

const res = await fetch_reqwest()
Deno.core.print(JSON.stringify(res, null, 2))
