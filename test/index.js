const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/mylibrary.dna.json")
const dna = Diorama.dna(dnaPath, 'mylibrary')

const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna,
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

diorama.registerScenario("description of example test", async (s, t, { alice, bob }) => {
  // Make a call to a Zome function
  // indicating the function, and passing it an input

  const title = "mein kampf1";

  const addr = await alice.call("books", "create_book", {"title":title})
  const result = await alice.call("books", "get_book", {"address": addr.Ok})
  const my_books = await alice.call("books","list_my_books" , { })
   
  const book = JSON.parse(my_books.Ok[0].App[1])

  // check for equality of the actual and expected results
  t.deepEqual(result, { Ok: { App: [ 'book', '{"title":"'+title+'","owner":"'+alice.agentAddress+'"}' ] } })
  t.equal(book.title,title)
})

diorama.run()
