use std::{error::Error, process::Command};

// public function to get jarm fingerprint for domain
pub fn get_jarm_fingerprint(domain: &str) -> Result<String, Box<dyn Error>> {
    // execute JARM.py in subprocess
    let output = Command::new("python3")
        .args(&["jarm.py", domain])
        .output()?;

    // if JARM success
    if output.status.success() {
        // parse output (Vec<u8> into String)
        let raw_output = String::from_utf8(output.stdout)?;

        // get JARM fingerprint by parsing command output
        // JARM fingerprint is last item in the parsed string
        let jarm_fingerprint_option = raw_output.split_whitespace().last();

        // init mutable, empty string literal to hold JARM fingerprint
        #[allow(unused_assignments)]
        let mut jarm_fingerprint: &str = "";

        // handle option
        match jarm_fingerprint_option {
            Some(j) => {
                jarm_fingerprint = j;
            }
            None => {
                jarm_fingerprint = "";
            }
        };

        Ok(jarm_fingerprint.to_string())
    // if JARM error
    } else {
        // parse output (Vec<u8> into String)
        let err = String::from_utf8(output.stderr)?;
        error_chain::bail!("Error generating JARM fingerprint:\n {}", err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_jarm_fingerprint() -> Result<(), Box<dyn std::error::Error>> {
        let cnn_fingerprint = "29d29d00029d29d21c29d29d29d29d7803e63b02b0ffde37ab35a15e335653";

        assert_eq!(get_jarm_fingerprint("cnn.com")?, cnn_fingerprint);
        Ok(())
    }
}
