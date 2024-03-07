
#[derive(Debug, Clone)]
enum NodeType {
    Litteral(char),
    Wildcard,
    Group(Vec<Node>)
}

use NodeType::*;

#[derive(Debug, Clone)]
struct Node {
    t: NodeType,
    zero: bool,
    multiple: bool,
}

impl Node {
    pub fn new(t: NodeType) -> Self {
        Node { t: t, zero: false, multiple: false }
    }
}

#[derive(Debug)]
struct Regex {
    nodes: Vec<Node>,
}

#[derive(Debug)]
enum RegexErr {
    InvalidParenthesis,
    NoCharToEffect,
}

impl Regex {
    pub fn from_string(input_string: &str) -> Result<Self, RegexErr> {
        let mut i = 0;

        
        let mut stack: Vec<Vec<Node>> = Vec::new();
        
        stack.push(Vec::new());
        
        while i < input_string.len() {
            
            
            match input_string.chars().nth(i).unwrap() {
                '.' => stack.last_mut().unwrap().push(Node::new(Wildcard)),
                '*' => {
                    stack.last_mut().unwrap().last_mut().ok_or(RegexErr::NoCharToEffect)?.zero = true;
                    stack.last_mut().unwrap().last_mut().ok_or(RegexErr::NoCharToEffect)?.multiple = true;
                },
                '?' => stack.last_mut().unwrap().last_mut().ok_or(RegexErr::NoCharToEffect)?.zero = true,
                '+' => stack.last_mut().unwrap().last_mut().ok_or(RegexErr::NoCharToEffect)?.multiple = true,
                '\\' => {
                    i += 1;
                    stack.last_mut().unwrap().push(Node::new(Litteral(input_string.chars().nth(i).unwrap())));
                },
                '(' => stack.push(Vec::new()),
                ')' => {
                    let g = stack.pop().ok_or(RegexErr::InvalidParenthesis)?;
                    stack.last_mut().ok_or(RegexErr::InvalidParenthesis)?.push(Node::new(Group(g)));
                },
                x => stack.last_mut().unwrap().push(Node::new(Litteral(x)))
            }
            
            i += 1;
        }
        Ok(Regex { nodes: stack.last().unwrap().clone() })
    }

    fn check_node(node: &Node, input_string: &str, index: usize) -> (bool, usize) {
        match &node.t {
            NodeType::Litteral(x) => if input_string.chars().nth(index) == Some(*x) {(true, index+1)} else {(false, index)},
            NodeType::Wildcard => (true, index+1),
            NodeType::Group(x) => Self::check_group(x, input_string, index),
        }
    }

    fn check_group(g: &Vec<Node>, input_string: &str, index: usize) -> (bool, usize) {
        let mut index = index.clone();
        let mut result: bool;

        for (e, i) in g.iter().enumerate() {
            if i.multiple {
                let mut stack: Vec<usize> = Vec::new();
                let mut temp_index = index.clone();

                while {
                    (result, temp_index) = Self::check_node(i, input_string, temp_index);
                    result
                } {stack.push(temp_index);}

                if stack.is_empty() & !i.zero {
                    println!("empt");
                    return (false, index);
                }

                for n in stack.iter().rev() {
                    (result, temp_index) = Self::check_group(&g[(e+1)..].to_vec(), input_string, *n);

                    if result {return (result, temp_index)}
                }
            }
            (result, index) = Self::check_node(i, input_string, index);
            if !result & !i.zero {return (false, index)}
        }
        (true, index)
    }

    pub fn test(self, input_string: &str) -> bool {
        (true, input_string.len()) == Self::check_group(&self.nodes, input_string, 0)
    }
}

fn main() {
    match Regex::from_string(".*.*") {
        Ok(x) => {
            let result = x.test("");
            println!("{}", result);
        },
        Err(x) => println!("{:?}", x),
    }

}
