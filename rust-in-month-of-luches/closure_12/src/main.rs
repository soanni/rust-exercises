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

fn main() {
    let companies = vec![
        Company::new("Company 1", ""),
        Company::new("Company 2", "CEO 2"),
        Company::new("Company 3", ""),
        Company::new("Company 4", "CEO 4"),
    ];

    let all_ceos = companies
        .iter()
        .map(|c| c.get_ceo().ok_or("No CEO found"))
        .collect::<Vec<Result<String, &str>>>();

    println!("{all_ceos:?}");
}
