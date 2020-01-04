use crate::parser::io::GherkinBuffer;
use crate::parser::dialect::Dialect;
use std::fs::File;

pub const RULE_LENGTH: usize = 33;

pub enum Rule {
    None,
    EOF,                // #EOF
    Empty,              // #Empty
    Comment,            // #Comment
    TagLine,            // #TagLine
    FeatureLine,        // #FeatureLine
    RuleLine,           // #RuleLine
    BackgroundLine,     // #BackgroundLine
    ScenarioLine,       // #ScenarioLine
    ExamplesLine,       // #ExamplesLine
    StepLine,           // #StepLine
    DocStringSeparator, // #DocStringSeparator
    TableRow,           // #TableRow
    Language,           // #Language
    Other,              // #Other
    GherkinDocument,    // GherkinDocument! := Feature?
    Feature,            // Feature! := FeatureHeader Background? ScenarioDefinition* Rule*
    FeatureHeader,      // FeatureHeader! := #Language? Tags? #FeatureLine DescriptionHelper
    Rule,               // Rule! := RuleHeader Background? ScenarioDefinition*
    RuleHeader,         // RuleHeader! := #RuleLine DescriptionHelper
    Background,         // Background! := #BackgroundLine DescriptionHelper Step*
    ScenarioDefinition, // ScenarioDefinition! := Tags? Scenario
    Scenario,           // Scenario! := #ScenarioLine DescriptionHelper Step* ExamplesDefinition*
    ExamplesDefinition, // ExamplesDefinition! [#Empty|#Comment|#TagLine-&gt;#ExamplesLine] := Tags? Examples
    Examples,           // Examples! := #ExamplesLine DescriptionHelper ExamplesTable?
    ExamplesTable,      // ExamplesTable! := #TableRow #TableRow*
    Step,               // Step! := #StepLine StepArg?
    StepArg,            // StepArg := (DataTable | DocString)
    DataTable,          // DataTable! := #TableRow+
    DocString,          // DocString! := #DocStringSeparator #Other* #DocStringSeparator
    Tags,               // Tags! := #TagLine+
    DescriptionHelper,  // DescriptionHelper := #Empty* Description? #Comment*
    Description,        // Description! := #Other+
}

impl Rule {
    pub fn to_usize(&self) -> usize {
        match *self {
            Rule::None => 0,
            Rule::EOF => 1,
            Rule::Empty => 2,
            Rule::Comment => 3,
            Rule::TagLine => 4,
            Rule::FeatureLine => 5,
            Rule::RuleLine => 6,
            Rule::BackgroundLine => 7,
            Rule::ScenarioLine => 8,
            Rule::ExamplesLine => 9,
            Rule::StepLine => 10,
            Rule::DocStringSeparator => 11,
            Rule::TableRow => 12,
            Rule::Language => 13,
            Rule::Other => 14,
            Rule::GherkinDocument => 15,
            Rule::Feature => 16,
            Rule::FeatureHeader => 17,
            Rule::Rule => 18,
            Rule::RuleHeader => 19,
            Rule::Background => 20,
            Rule::ScenarioDefinition => 21,
            Rule::Scenario => 22,
            Rule::ExamplesDefinition => 23,
            Rule::Examples => 24,
            Rule::ExamplesTable => 25,
            Rule::Step => 26,
            Rule::StepArg => 27,
            Rule::DataTable => 28,
            Rule::DocString => 29,
            Rule::Tags => 30,
            Rule::DescriptionHelper => 31,
            Rule::Description => 32
        }
    }
}

pub struct GherkinParser {
    gherkin_buffer: GherkinBuffer<File>,
    dialect: Dialect,
}

impl GherkinParser {
    pub fn create(gherkin_buffer: GherkinBuffer<File>, dialect: Dialect) -> GherkinParser {
        GherkinParser {
            gherkin_buffer: gherkin_buffer,
            dialect: dialect
        }
    }
}
