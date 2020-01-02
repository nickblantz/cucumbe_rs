pub enum StepType { Given, When, Then, And, But }

pub struct Table {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub position: (usize, usize),
}

pub struct Step {
    pub step_type: StepType,
    pub raw_content: String,
    pub doc_string: Option<String>,
    pub data_table: Option<Table>,
    pub position: (usize, usize),
}

pub struct Examples {
    pub table: Table,
    pub position: (usize, usize),
}

pub struct Background {
    pub steps: Vec<Step>,
    pub position: (usize, usize),
}

pub struct Scenario {
    pub name: String,
    pub steps: Vec<Step>,
    pub examples: Option<Examples>,
    pub tags: Option<Vec<String>>,
    pub position: (usize, usize),
}

pub struct Rule {
    pub name: String,
    pub scenarios: Vec<Scenario>,
    pub tags: Option<Vec<String>>,
    pub position: (usize, usize),
}

pub struct Feature {
    pub name: String,
    pub description: Option<String>,
    pub background: Option<Background>,
    pub rules: Option<Vec<Rule>>,
    pub scenarios: Option<Vec<Scenario>>,
    pub tags: Option<Vec<String>>,
    pub position: (usize, usize),
}