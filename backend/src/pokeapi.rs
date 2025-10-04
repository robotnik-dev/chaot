use anyhow::{Context, anyhow};
use log::info;
use serde_json::Value;

use crate::{csv_record, error::Result, index::Index, name::Name};

#[derive(Debug, Clone)]
pub struct PokeApi;

impl PokeApi {
    pub async fn get_id(base_url: &str, language_url: &str, name: &Name) -> Result<usize> {
        let name = name.0.replace(" ", "-");
        let url = format!("{}{}/", base_url, name);
        info!("Getting ID for name: {name} on URL: {url}");
        let resp = reqwest::get(url.clone())
            .await
            .with_context(|| format!("Couldn't reach URL: {}", url.clone()))?
            .text()
            .await
            .context("Couldn't fetch text from URL")?;
        if resp.to_lowercase() == "not found" {
            // try german name
            info!("No english version of name: {name} found, trying german version...");
            let resp = reqwest::get(language_url)
                .await
                .with_context(|| format!("Couldn't reach URL: {}", language_url))?
                .text()
                .await
                .context("Couldn't fetch text from URL")?;

            // get csv data
            info!("Parsing CSV data");
            let mut pkm_species_id = None;
            let mut rdr = csv::Reader::from_reader(resp.as_bytes());
            for result in rdr.deserialize() {
                let record: csv_record::Record = result.context("Couldn't parse csv record")?;
                if name.to_lowercase() == record.name.to_lowercase() {
                    pkm_species_id = Some(record.pokemon_species_id);
                }
            }
            match pkm_species_id {
                Some(id) => {
                    let mut rdr = csv::Reader::from_reader(resp.as_bytes());
                    for result in rdr.deserialize() {
                        let record: csv_record::Record =
                            result.context("Couldn't parse csv record")?;
                        if id == record.pokemon_species_id && record.local_language_id == 9 {
                            info!("English CSV record found with species ID: {id}");
                            let eng_name = record.name.replace(" ", "-");
                            let url = format!("{}{}/", base_url, eng_name);
                            info!("Trying to find german equivalent for {eng_name} at URL {url}");
                            let resp = reqwest::get(url.clone())
                                .await
                                .with_context(|| format!("Couldn't reach URL: {}", url.clone()))?
                                .text()
                                .await
                                .context("Couldn't fetch text from URL")?;
                            let value: Value = serde_json::from_str(&resp)
                                .context("Couldn't parse URL response into JSON")?;
                            let id = value["id"].to_string().parse::<usize>()?;
                            return Ok(id);
                        }
                    }
                    Err(anyhow!("No english CSV record found with species ID: {id}",).into())
                }
                None => Err(anyhow!("No german or english version found for name: {name}").into()),
            }
        } else {
            let value: Value = serde_json::from_str(&resp)?;
            let id = value["id"].to_string().parse::<usize>()?;
            Ok(id)
        }
    }

    pub async fn get_names(
        index: &Index,
        base_url: &str,
        language_url: &str,
    ) -> Result<Vec<String>> {
        let mut names = vec![];
        let url = format!("{}{}/", base_url, index.0);
        info!("requesting url: {url}");
        let api_resp = reqwest::get(url.clone()).await?.text().await?;
        if api_resp.to_lowercase() == "not found" {
            return Err(anyhow!("Nothing found for id: {index} at {url}",).into());
        } else {
            let value: Value = serde_json::from_str(&api_resp)?;
            let eng_name = value["species"]["name"].to_string().replace("\"", "");
            names.push(eng_name.to_lowercase());
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
                        names.push(ger_name.to_lowercase());
                        found = true;
                    }
                }
                if !found {
                    return Err(anyhow!("Couldn't find german name in csv file:URL: {language_url}\nspecies ID: {id}\nenglish name: {}", names[0],).into());
                }
            }
            None => {
                return Err(anyhow!(
                    "Couldn't find english name in csv file: URL: {language_url}\nName: `{}`",
                    names[0],
                )
                .into());
            }
        }
        Ok(names)
    }
}
