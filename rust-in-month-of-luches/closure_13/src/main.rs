struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            _ => Some(ceo.to_string()),
        };

        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn get_datetime() -> String {
    "Sept 12 02.05am".to_string()
}

fn main() {
    let companies = vec![
        Company::new("Company 1", ""),
        Company::new("Company 2", "CEO 2"),
        Company::new("Company 3", ""),
        Company::new("Company 4", "CEO 4"),
    ];

    let all_ceos = companies
        .iter()
        .map(|c| {
            c.get_ceo().ok_or_else(|| {
                let err_msg = format!("Error happened for company {}", c.name);
                println!("{} error occured at {}", err_msg, get_datetime());
                err_msg
            })
        })
        .collect::<Vec<Result<String, String>>>();

    all_ceos
        .iter()
        .filter(|c| c.is_ok())
        .for_each(|ceo| println!("{ceo:?}"));
}
