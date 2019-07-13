const { connect } = require('@holochain/hc-web-client')

connect({url:"ws://localhost:8888"}).then(async ({callZome, close}) => {
    
    const result = r => JSON.parse(r).Ok 

    try {
        let bookid = await callZome('test-instance', 'books', 'create_book')({title:"alice in wonderland"});
        console.log(result(bookid))
        book_list = await callZome('test-instance', 'books', 'list_my_books')({});
        console.log(result(book_list))
    } catch (err) {
        console.log("got error",err)
    }
})