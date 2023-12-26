use clap::Parser;
use encryptor::password::generate_password;
use prettytable::{row, Table};
use std::fmt::Debug;
use url::{ParseError, Url};

#[derive(Parser, Debug)]
#[clap(version,author,about,long_about=None)]
struct Args {
    path: String,
    username: String,

    #[clap(default_value = "16")]
    length: usize,
}

fn main() {
    let args = Args::parse();

    let (path, username, length) = (args.path, args.username, args.length);

    let domain_name = get_domain_name(path).unwrap();
    let seed = format!("{}{}", domain_name, username);
    let passwd = generate_password(&seed[..], length).unwrap();

    let mut table = Table::new();
    table.add_row(row!["tag", "username", "passwd"]);
    table.add_row(row![domain_name, username, passwd]);
    table.printstd();
}

fn get_domain_name(url_string: String) -> Result<String, ParseError> {
    let url = Url::parse(&url_string)?;
    Ok(url.host_str().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_domain_name() {
        let url = "https://www.google.com".to_string();
        let domain_name = get_domain_name(url).unwrap();
        assert_eq!(domain_name, "www.google.com");
    }
    #[test]
    fn test_get_domain_name_with_port() {
        let url = "https://www.google.com:8080".to_string();
        let domain_name = get_domain_name(url).unwrap();
        assert_eq!(domain_name, "www.google.com");
    }
    #[test]
    fn test_get_domain_name_with_path() {
        let url = "https://docs.rs/url/latest/url/".to_string();
        let domain_name = get_domain_name(url).unwrap();
        assert_eq!(domain_name, "docs.rs");
    }
    #[test]
    fn test_get_domain_name_with_path_and_port() {
        let url = "https://docs.rs:8080/url/latest/url/".to_string();
        let domain_name = get_domain_name(url).unwrap();
        assert_eq!(domain_name, "docs.rs");
    }

    // #[test]
    // #[should_panic]
    // fn test_get_domain_name_should_panic() {
    //     let url = "https:/docs.rs:8080/url/latest/url/".to_string();
    //     let domain_name = GetDomainName(url).unwrap();
    //     assert_eq!(domain_name, "docs.rs");
    // }
}
