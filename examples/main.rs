use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // domains vector
    let domains: Vec<&str> = vec!["cnn.com", "espn.com", "aol.com", "facebook.com"];

    // declare new hashmap to hold domain -> JARM fingerprint
    let mut domain_jarm_map = HashMap::new();

    // loop over domains
    for domain in domains {
        println!("Generating JARM fingerprint for: {}", domain);
        // get JARM fingerprint for domain
        let jarm_fingerprint = jarm_rs::get_jarm_fingerprint(domain)?;

        // add to hashmap
        domain_jarm_map.insert(domain, jarm_fingerprint);
    }

    // pretty print map results
    println!("{:#?}", domain_jarm_map);

    Ok(())
}
