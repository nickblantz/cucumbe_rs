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

pub struct AstNode {
    rule: Rule,
    children: Vec<AstNode>,
}