### jarm_rs

[![Latest Version](https://img.shields.io/crates/v/jarm_rs.svg)](https://crates.io/crates/jarm_rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/waymobetta/jarm_rs)

generate a JARM fingerprint for a domain or list of domains
- this is a wrapper around a python script created by Salesforce

### requirements
- [python3](https://www.python.org/downloads/)

### usage
_Cargo.toml_
```toml
[dependencies]
jarm_rs = "0.1.3"
```

### example
```zsh
cargo run --example main
```

![EXAMPLE](https://user-images.githubusercontent.com/17755587/107860883-d2be4780-6df6-11eb-8dda-61b11a8bbe9c.png)

### notes
- code likely has much room for improvement; pull requests appreciated!

### resources
- official [JARM](https://github.com/salesforce/jarm) repository from Salesforce

[MIT](LICENSE)
