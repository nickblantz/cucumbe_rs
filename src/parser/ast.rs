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

    pub fn add(&mut self, rule: Rule, node: AstNode) {
        match self.children[rule.to_usize()] {
            Some(node_list) => { node_list.asMut().push(node) },
            None => { self.children[rule.to_usize()] = Some(vec![node]) }
        }
    }
}

impl AstBuilder {
    pub fn new(id_generator: Option<()>) -> AstBuilder {
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
        self.stack.push(AstNode::new(rule));
    }

    pub fn end_rule(&mut self) {
        // let node = self.stack.pop();
        // self.stack.last.add(node.rule, )
        // let x: str = 4;
        // let test = Vec::default();
        // test.first()

        // current_node.add(node.rule_type, transform_node(node))
    }

    



}