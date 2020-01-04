use crate::parser::parser::{Rule, RULE_LENGTH};

type AstNodeArrayOfRules = [Option<Vec<AstNode>>; RULE_LENGTH];

pub struct AstNode {
    pub rule: Rule,
    pub children: AstNodeArrayOfRules
}

pub struct AstBuilder {
    id_generator: Option<()>,
    stack: Vec<AstNode>,
    comments: Vec<String>
}

impl AstNode {
    pub fn new(rule: Rule) -> AstNode {
        AstNode {
            rule: rule,
            children: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None]
        }
    }

    pub fn new_empty() {

    }
    
    fn to_array() -> AstNodeArrayOfRules {

    }
}

impl AstBuilder {
    pub fn new(id_generator: Some<()>) {
        AstBuilder {
            id_generator: id_generator,
            stack: vec![AstNode::new(Rule::None)],
            comments: Vec::<String>::new()
        }
    }

    pub fn reset(&mut self) {
        self.stack = vec![AstNode::new(Rule::None)];
        self.comments = Vec::<String>::new();
    }

    pub fn start_rule(&mut self, rule: Rule) {
        self.comments.push(rule);
    }

    pub fn end_rule(&mut self) {
        let node = stack.pop();
        let x: str = 4;
        // let test = Vec::default();
        // test.first()

        // current_node.add(node.rule_type, transform_node(node))
    }

    



}