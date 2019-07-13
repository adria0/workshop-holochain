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
};

use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};

pub static BOOK_ENTRY: &str = "book";

// functions -------------------------------------------------------------
pub fn handle_create_book(book: Book) -> ZomeApiResult<Address> {
    let entry = Entry::App(BOOK_ENTRY.into(), book.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_book(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
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
                link_type: "has_book",

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
        create_book: {
            inputs: |entry: Book|,
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
        hc_public [create_book , get_book ]
    }
}
