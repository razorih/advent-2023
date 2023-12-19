use std::collections::HashMap;

use advent::read_input;

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get(&self, field: Field) -> u32 {
        match field {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s,
        }
    }

    fn score(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
enum RuleResult {
    Accept,
    Reject,
    Next(String),
}

struct Rule {
    check: Box<dyn Fn(&Part) -> bool>,
    outcome: RuleResult,
}

fn main() -> anyhow::Result<()> {
    let input = read_input()?;
    let (workflows, parts) = parse(&input);

    println!("workflows: {workflows:?}");
    println!("parts: {parts:?}");

    let mut accept_score = 0;

    let first = &workflows["in"];
    for part in parts {
        let mut flow = first.iter();
        while let Some(rule) = flow.next() {
            if (rule.check)(&part) {
                // println!("matched rule!");
                match &rule.outcome {
                    RuleResult::Accept => { 
                        accept_score += part.score();
                        break
                    },
                    RuleResult::Reject => break,
                    RuleResult::Next(next_flow) => flow = workflows[next_flow].iter(),
                }
            }
        }
    }

    println!("Silver: {}", accept_score);

    Ok(())
}

fn parse(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    let mut lines = input.trim().lines();

    // Lines have workflows until the first empty line
    for workflow_line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let (name, rest) = workflow_line.split_once('{').unwrap();
        let rest = rest.strip_suffix('}').unwrap();

        let mut rules = Vec::new();
        for rule in rest.split(',') {
            // Rule Cases:
            // - Outcome conditional on field:
            //     <field><op><number>:<outcome>
            // - Unconditional outcome
            //     <outcome>
            if let Some((field_op_number, outcome)) = rule.split_once(':') {
                let outcome = match outcome {
                    "A"  => RuleResult::Accept,
                    "R"  => RuleResult::Reject,
                    next => RuleResult::Next(next.to_string())
                };

                let mut parts = field_op_number.match_indices(['>', '<']);
                let (op_idx, op) = parts.next().unwrap();
                let field = &field_op_number[..op_idx];
                let amount = &field_op_number[op_idx+1..];

                let field = Field::from_str(field);
                let amount = amount.parse::<u32>().unwrap();

                let check_fn: Box<dyn Fn(&Part) -> bool> = match op {
                    "<" => Box::new(move |part| part.get(field) < amount),
                    ">" => Box::new(move |part| part.get(field) > amount),
                    _ => panic!("Invalid op"),
                };
                rules.push(Rule {
                    check: check_fn,
                    outcome,
                });
            } else {
                // unconditionally pass on to some other rule or accept/reject
                let outcome = match rule {
                    "A" => RuleResult::Accept,
                    "R" => RuleResult::Reject,
                    next => RuleResult::Next(next.to_string())
                };

                rules.push(Rule {
                    check: Box::new(|_| true),
                    outcome,
                });
            }
        }

        workflows.insert(name.to_string(), rules);
    }

    // Rest of lines contain parts.
    // Note: The empty line has been consumed by `take_while()`
    for part_line in lines {
        let part_line = part_line.strip_prefix('{').and_then(|s| s.strip_suffix('}')).unwrap();
        let mut components = part_line.split(',')
            .map(|comp| comp.split_once('=').unwrap().1)
            .map(|n| n.parse::<u32>().unwrap());

        // Assume that components are always yielded in order (xmas)
        parts.push(Part {
            x: components.next().unwrap(),
            m: components.next().unwrap(),
            a: components.next().unwrap(),
            s: components.next().unwrap(),
        });
    }

    (workflows, parts)
}

#[derive(Debug, Clone, Copy)]
enum Field { X, M, A, S }

impl Field {
    fn from_str(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("invalid field '{s}'"),
        }
    }
}

impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule")
            .field("outcome", &self.outcome)
            .finish()
    }
}
