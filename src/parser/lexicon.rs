pub trait IntoToken {
    fn get_symbol(&self) -> String;
    fn get_value(&self) -> &String;
}

impl std::fmt::Debug for dyn IntoToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{{ {} '{}' }}", self.get_symbol(), *self.get_value())
    }
}

pub enum Rules {
    None,
    _EOF,                // #EOF
    _Empty,              // #Empty
    _Comment,            // #Comment
    _TagLine,            // #TagLine
    _FeatureLine,        // #FeatureLine
    _RuleLine,           // #RuleLine
    _BackgroundLine,     // #BackgroundLine
    _ScenarioLine,       // #ScenarioLine
    _ExamplesLine,       // #ExamplesLine
    _StepLine,           // #StepLine
    _DocStringSeparator, // #DocStringSeparator
    _TableRow,           // #TableRow
    _Language,           // #Language
    _Other,              // #Other
    GherkinDocument,     // GherkinDocument! := Feature?
    Feature,             // Feature! := FeatureHeader Background? ScenarioDefinition* Rule*
    FeatureHeader,       // FeatureHeader! := #Language? Tags? #FeatureLine DescriptionHelper
    Rule,                // Rule! := RuleHeader Background? ScenarioDefinition*
    RuleHeader,          // RuleHeader! := #RuleLine DescriptionHelper
    Background,          // Background! := #BackgroundLine DescriptionHelper Step*
    ScenarioDefinition,  // ScenarioDefinition! := Tags? Scenario
    Scenario,            // Scenario! := #ScenarioLine DescriptionHelper Step* ExamplesDefinition*
    ExamplesDefinition,  // ExamplesDefinition! [#Empty|#Comment|#TagLine-&gt;#ExamplesLine] := Tags? Examples
    Examples,            // Examples! := #ExamplesLine DescriptionHelper ExamplesTable?
    ExamplesTable,       // ExamplesTable! := #TableRow #TableRow*
    Step,                // Step! := #StepLine StepArg?
    StepArg,             // StepArg := (DataTable | DocString)
    DataTable,           // DataTable! := #TableRow+
    DocString,           // DocString! := #DocStringSeparator #Other* #DocStringSeparator
    Tags,                // Tags! := #TagLine+
    DescriptionHelper,   // DescriptionHelper := #Empty* Description? #Comment*
    Description,         // Description! := #Other+
}
