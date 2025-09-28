use serde_json::Value;

use crate::{ApplicationError, csv_record, index::Index, name::Name};

#[derive(Debug, Clone)]
pub struct PokeApi;

impl PokeApi {
    pub async fn get_id(
        base_url: &str,
        language_url: &str,
        name: &Name,
    ) -> Result<usize, ApplicationError> {
        match name {
            Name::En(name) => {
                let name = name.replace(" ", "-");
                let url = format!("{}{}/", base_url, name);
                let resp = reqwest::get(url).await?.text().await?;
                let value: Value = serde_json::from_str(&resp)?;
                let id = value["id"].to_string().parse::<usize>()?;
                Ok(id)
            }
            Name::De(name) => {
                let resp = reqwest::get(language_url).await?.text().await?;

                // get csv data
                let mut pkm_species_id = None;
                let mut rdr = csv::Reader::from_reader(resp.as_bytes());
                for result in rdr.deserialize() {
                    let record: csv_record::Record = result?;
                    if name.to_lowercase() == record.name.to_lowercase() {
                        pkm_species_id = Some(record.pokemon_species_id);
                    }
                }
                match pkm_species_id {
                    Some(id) => {
                        let mut rdr = csv::Reader::from_reader(resp.as_bytes());
                        for result in rdr.deserialize() {
                            let record: csv_record::Record = result?;
                            if id == record.pokemon_species_id && record.local_language_id == 9 {
                                let eng_name = record.name.replace(" ", "-");
                                let url = format!("{}{}/", base_url, eng_name);
                                let resp = reqwest::get(url).await?.text().await?;
                                let value: Value = serde_json::from_str(&resp)?;
                                let id = value["id"].to_string().parse::<usize>()?;
                                return Ok(id);
                            }
                        }
                        Err(ApplicationError::NotFound(
                            "Pokemon not found while fetching english and german name".into(),
                        ))
                    }
                    None => Err(ApplicationError::NotFound(
                        "csv entry for german name not found".into(),
                    )),
                }
            }
        }
    }

    pub async fn get_names(
        index: &Index,
        base_url: &str,
        language_url: &str,
    ) -> Result<Vec<String>, ApplicationError> {
        let mut names = vec![];
        let url = format!("{}{}/", base_url, index.0);
        let api_resp = reqwest::get(url).await?.text().await?;
        if api_resp.to_lowercase() == "not found" {
            return Err(ApplicationError::NotFound(
                "Pokemon not found while fetching english name".into(),
            ));
        } else {
            let value: Value = serde_json::from_str(&api_resp)?;
            let eng_name = value["name"].to_string().replace("\"", "");
            names.push(eng_name);
        };

        // get german name
        let lan_resp = reqwest::get(language_url).await?.text().await?;

        // get csv data
        let mut pkm_species_id = None;
        let mut rdr = csv::Reader::from_reader(lan_resp.as_bytes());
        for result in rdr.deserialize() {
            let record: csv_record::Record = result?;
            if names[0].to_lowercase() == record.name.to_lowercase() {
                pkm_species_id = Some(record.pokemon_species_id);
            }
        }
        match pkm_species_id {
            Some(id) => {
                let mut found = false;
                let mut rdr = csv::Reader::from_reader(lan_resp.as_bytes());
                for result in rdr.deserialize() {
                    let record: csv_record::Record = result?;
                    if id == record.pokemon_species_id && record.local_language_id == 6 {
                        let ger_name = record.name;
                        names.push(ger_name);
                        found = true;
                    }
                }
                if !found {
                    return Err(ApplicationError::NotFound("Both names not found".into()));
                }
            }
            None => {
                return Err(ApplicationError::NotFound(
                    "csv entry for german name not found".into(),
                ));
            }
        }
        Ok(names)
    }
}
