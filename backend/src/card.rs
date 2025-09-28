use crate::{
    ApplicationError, BASE_URL, LANGUAGE_URL, book::Book, entry::Entry, index::Index, name::Name,
    page::Page, pokeapi::PokeApi, side::Side,
};
pub use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Card {
    index: Index,
    name_en: String,
    name_de: String,
    book: Book,
    page: Page,
    side: Side,
    entry: Entry,
}

impl Card {
    pub async fn try_from_index(index: Index) -> Result<Self, ApplicationError> {
        let names = PokeApi::get_names(&index, BASE_URL, LANGUAGE_URL).await?;
        let name_en = names[0].clone();
        let name_de = names[1].clone();
        let book = Book::from(&index);
        let page = Page::relative(&index);
        let side = Side::from(&index);
        let entry = Entry::new(&index, &Page::absolut(&index), &side);
        Ok(Self {
            index,
            name_en,
            name_de,
            book,
            page,
            side,
            entry,
        })
    }

    pub async fn try_from_name(name: Name) -> Result<Self, ApplicationError> {
        let id = PokeApi::get_id(BASE_URL, LANGUAGE_URL, &name).await?;
        let index = Index::try_from(id)?;
        Card::try_from_index(index).await
    }
}
