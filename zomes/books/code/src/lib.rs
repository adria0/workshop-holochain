#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    AGENT_ADDRESS
};

use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    link::LinkMatch
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};

pub static BOOK_ENTRY: &str = "book";
pub static HAS_BOOK_LINK_TYPE : &str = "has_book";

// functions -------------------------------------------------------------
pub fn handle_create_book(title: String) -> ZomeApiResult<Address> {
    let book = Book{ title, owner: AGENT_ADDRESS.to_owned() };
    let entry = Entry::App(BOOK_ENTRY.into(), book.into());
    let address = hdk::commit_entry(&entry)?;
    
    hdk::link_entries(&AGENT_ADDRESS, &address, HAS_BOOK_LINK_TYPE, "")?;

    Ok(address)
}

pub fn handle_get_book(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

pub fn handle_list_my_books() -> ZomeApiResult<Vec<Entry>> {
    hdk::get_links_and_load(&AGENT_ADDRESS, LinkMatch::Exactly(HAS_BOOK_LINK_TYPE), LinkMatch::Any)?
        .into_iter()
        .collect()
}


// entry defintions  -------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct Book {
    title: String,
    owner: Address,
}

fn book_definition() -> ValidatingEntryType {
    entry!(
        name: BOOK_ENTRY,
        description: "a book",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Book>| {            
            match _validation_data {
                 hdk::EntryValidationData::Create { entry, validation_data } => {
                     let is_owner = validation_data
                        .package
                        .chain_header
                        .provenances()
                        .iter()
                        .any(|p| p.source() == entry.owner);

                    let is_nazi = entry.title == "mein kampf";
                    
                    hdk::debug(format!("************************ nazi:{} debug:{}", is_nazi, is_owner))?;

                    if !is_owner || is_nazi {
                         Err("unauthorized".to_string())
                    } else {
                        Ok(())
                    }
                 },
                _ => Ok(()) 
            }
        },
        links: [
            from!(
                "%agent_id",
                link_type: HAS_BOOK_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]  
    ) 
}

// zome definition  -------------------------------------------------------------

define_zome! {
    entries: [
       book_definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        list_my_books: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<Entry>> |,
            handler: handle_list_my_books
        }
        create_book: {
            inputs: |title: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_book
        }
        get_book: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: handle_get_book
        }
    ]

    traits: {
        hc_public [create_book , get_book, list_my_books ]
    }
}
