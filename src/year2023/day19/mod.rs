#[derive(Debug, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
enum Operation {
    Greater,
    Lesser,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum EndDest {
    A,
    R,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum RuleDest {
    End(EndDest),
    OtherRule(String),
}

impl RuleDest {
    fn is_end(&self) -> bool {
        match self {
            Self::End(_) => true,
            Self::OtherRule(_) => false,
        }
    }
    fn is_approved(&self) -> bool {
        match self {
            Self::End(EndDest::A) => true,
            _ => false,
        }
    }
}

impl From<&str> for RuleDest {
    fn from(rule: &str) -> Self {
        match rule {
            "A" => RuleDest::End(EndDest::A),
            "R" => RuleDest::End(EndDest::R),
            a => RuleDest::OtherRule(a.to_string()),
        }
    }
}

impl Default for RuleDest {
    fn default() -> Self {
        Self::OtherRule(String::from("in"))
    }
}

#[derive(Debug, Clone)]
struct Rule {
    pub(crate) category: Category,
    pub(crate) operation: Operation,
    pub(crate) nbr: u64,
    pub(crate) dst: RuleDest,
}

impl Rule {
    fn is_valid(&self, rating: &Rating) -> bool {
        match self.category {
            Category::X => match self.operation {
                Operation::Greater => rating.x > self.nbr,
                Operation::Lesser => rating.x < self.nbr,
            },
            Category::M => match self.operation {
                Operation::Greater => rating.m > self.nbr,
                Operation::Lesser => rating.m < self.nbr,
            },
            Category::A => match self.operation {
                Operation::Greater => rating.a > self.nbr,
                Operation::Lesser => rating.a < self.nbr,
            },
            Category::S => match self.operation {
                Operation::Greater => rating.s > self.nbr,
                Operation::Lesser => rating.s < self.nbr,
            },
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let mut split = value.split(':');
        let mut start = split.next().expect("Should be present").chars();
        let category = match start.next() {
            Some('x') => Category::X,
            Some('m') => Category::M,
            Some('a') => Category::A,
            Some('s') => Category::S,
            _ => unreachable!(),
        };
        let operation = match start.next() {
            Some('<') => Operation::Lesser,
            Some('>') => Operation::Greater,
            _ => unreachable!(),
        };
        let nbr = start.fold(0, |acc, v| {
            acc * 10 + v.to_digit(10).expect("should be nbr") as u64
        });
        let dst = split.next().expect("should have dst").into();
        Self {
            category,
            operation,
            nbr,
            dst,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    pub(crate) name: RuleDest,
    pub(crate) rules: Vec<Rule>,
    pub(crate) default_dest: RuleDest,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let mut split = value.trim_end_matches('}').split('{');
        let name = RuleDest::OtherRule(split.next().expect("Should have a name").to_string());
        let mut rules_and_dest = split
            .next()
            .expect("Should have rules")
            .split(',')
            .peekable();
        let mut rules = vec![];
        let mut default_dest = Default::default();
        while let Some(rule) = rules_and_dest.next() {
            if rules_and_dest.peek().is_none() {
                default_dest = rule.into();
            } else {
                rules.push(rule.into())
            }
        }
        Self {
            name,
            rules,
            default_dest,
        }
    }
}

struct Rating {
    pub(crate) x: u64,
    pub(crate) m: u64,
    pub(crate) a: u64,
    pub(crate) s: u64,
}

#[derive(Debug, Clone)]
struct Bounds {
    pub(crate) x_lower_bound: u64,
    pub(crate) x_upper_bound: u64,
    pub(crate) m_lower_bound: u64,
    pub(crate) m_upper_bound: u64,
    pub(crate) a_lower_bound: u64,
    pub(crate) a_upper_bound: u64,
    pub(crate) s_lower_bound: u64,
    pub(crate) s_upper_bound: u64,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            x_lower_bound: 1,
            x_upper_bound: 4000,
            m_lower_bound: 1,
            m_upper_bound: 4000,
            a_lower_bound: 1,
            a_upper_bound: 4000,
            s_lower_bound: 1,
            s_upper_bound: 4000,
        }
    }
}

impl Bounds {
    fn eval_opportunities(self) -> u64 {
        let x_range = 0.max(self.x_upper_bound - self.x_lower_bound + 1);
        let m_range = 0.max(self.m_upper_bound - self.m_lower_bound + 1);
        let a_range = 0.max(self.a_upper_bound - self.a_lower_bound + 1);
        let s_range = 0.max(self.s_upper_bound - self.s_lower_bound + 1);
        x_range * m_range * a_range * s_range
    }

    fn is_valid(&self) -> bool {
        self.x_lower_bound < self.x_upper_bound
            && self.m_lower_bound < self.m_upper_bound
            && self.a_lower_bound < self.a_upper_bound
            && self.s_lower_bound < self.s_upper_bound
    }

    fn apply_rule(&mut self, rule: &Rule, default_bound: &mut Self) -> bool {
        let mut keep = true;
        match rule.category {
            Category::X => {
                match rule.operation {
                    Operation::Greater => {
                        self.x_lower_bound = self.x_lower_bound.max(rule.nbr + 1);
                        default_bound.x_upper_bound = self.x_lower_bound - 1;
                    }
                    Operation::Lesser => {
                        self.x_upper_bound = self.x_upper_bound.min(rule.nbr - 1);
                        default_bound.x_lower_bound = self.x_upper_bound + 1;
                    }
                }
                if self.x_lower_bound >= self.x_upper_bound {
                    keep = false
                }
            }
            Category::M => {
                match rule.operation {
                    Operation::Greater => {
                        self.m_lower_bound = self.m_lower_bound.max(rule.nbr + 1);
                        default_bound.m_upper_bound = self.m_lower_bound - 1;
                    }
                    Operation::Lesser => {
                        self.m_upper_bound = self.m_upper_bound.min(rule.nbr - 1);
                        default_bound.m_lower_bound = self.m_upper_bound + 1;
                    }
                }
                if self.m_lower_bound >= self.m_upper_bound {
                    keep = false
                }
            }
            Category::A => {
                match rule.operation {
                    Operation::Greater => {
                        self.a_lower_bound = self.a_lower_bound.max(rule.nbr + 1);
                        default_bound.a_upper_bound = self.a_lower_bound - 1;
                    }
                    Operation::Lesser => {
                        self.a_upper_bound = self.a_upper_bound.min(rule.nbr - 1);
                        default_bound.a_lower_bound = self.a_upper_bound + 1;
                    }
                }
                if self.a_lower_bound >= self.a_upper_bound {
                    keep = false
                }
            }
            Category::S => {
                match rule.operation {
                    Operation::Greater => {
                        self.s_lower_bound = self.s_lower_bound.max(rule.nbr + 1);
                        default_bound.s_upper_bound = self.s_lower_bound - 1;
                    }
                    Operation::Lesser => {
                        self.s_upper_bound = self.s_upper_bound.min(rule.nbr - 1);
                        default_bound.s_lower_bound = self.s_upper_bound + 1;
                    }
                }
                if self.s_lower_bound >= self.s_upper_bound {
                    keep = false
                }
            }
        }
        keep
    }
}

impl From<&str> for Rating {
    fn from(value: &str) -> Self {
        let remove = value.replace('{', "").replace('}', "");
        let mut split = remove.split(',');
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        while let Some(v) = split.next() {
            let mut inner = v.split('=');
            let target = inner.next();
            let nbr = inner
                .next()
                .expect("nbr should be present")
                .parse()
                .expect("Is a nbr");
            match target {
                Some("x") => x = nbr,
                Some("m") => m = nbr,
                Some("a") => a = nbr,
                Some("s") => s = nbr,
                _ => unreachable!(),
            };
        }
        Self { x, m, a, s }
    }
}

impl Rating {
    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

struct Heap {
    pub(crate) workflows: Vec<Workflow>,
    pub(crate) ratings: Vec<Rating>,
}

impl From<&str> for Heap {
    fn from(value: &str) -> Self {
        let removed = value.replace('\r', "");
        let mut workflows_and_ratings = removed.split("\n\n");
        let workflows = workflows_and_ratings
            .next()
            .expect("Should have workflows")
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.into())
            .collect();
        let ratings = workflows_and_ratings
            .next()
            .expect("Should have ratings")
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.into())
            .collect();
        Self { workflows, ratings }
    }
}

impl Heap {
    fn eval_rating(&self, current_rule: RuleDest, rating: &Rating) -> bool {
        match current_rule {
            RuleDest::End(EndDest::A) => true,
            RuleDest::End(EndDest::R) => false,
            name => {
                let current_workflow = self
                    .workflows
                    .iter()
                    .find(|w| w.name == name)
                    .expect("Should have this workflow");
                let new_dest = current_workflow
                    .rules
                    .iter()
                    .find(|r| r.is_valid(rating))
                    .map(|r| r.dst.clone())
                    .unwrap_or(current_workflow.default_dest.clone());
                self.eval_rating(new_dest, rating)
            }
        }
    }

    fn count_accepted(&self) -> u64 {
        self.ratings
            .iter()
            .filter_map(|r| {
                if self.eval_rating(Default::default(), r) {
                    Some(r.sum())
                } else {
                    None
                }
            })
            .sum()
    }

    fn explore_path(
        &self,
        current: &RuleDest,
        bound: Bounds,
        mut path: Vec<RuleDest>,
    ) -> Vec<Bounds> {
        if current.is_end() {
            if current.is_approved() {
                vec![bound]
            } else {
                vec![]
            }
        } else {
            let curr = self
                .workflows
                .iter()
                .find(|w| &w.name == current)
                .expect("Should have this workflow");
            let mut default_bound = bound.clone();
            let mut ret = vec![];
            for r in curr.rules.iter() {
                let mut new_bound = default_bound.clone();
                if new_bound.apply_rule(r, &mut default_bound) {
                    let mut new_path = path.clone();
                    new_path.push(r.dst.clone());
                    ret.append(&mut self.explore_path(&r.dst, new_bound, new_path));
                };
            }
            if default_bound.is_valid() {
                path.push(curr.default_dest.clone());
                ret.append(&mut self.explore_path(&curr.default_dest, default_bound, path));
            }
            ret
        }
    }

    fn explore_paths(&self) -> u64 {
        let current = RuleDest::OtherRule(String::from("in"));
        let bound = Bounds::default();
        let ret = self.explore_path(&current, bound, vec![current.clone()]);
        ret.into_iter()
            .fold(0, |acc, r| acc + r.eval_opportunities())
    }
}

pub(crate) fn eval_file(file: &str) -> u64 {
    let heap: Heap = file.into();
    heap.count_accepted()
}

pub(crate) fn eval_file_2(file: &str) -> u64 {
    let heap: Heap = file.into();
    let ret = heap.explore_paths();
    ret
}

pub(crate) fn print_sol_1(file: &str) {
    print!("res : {}", eval_file(file));
}
pub(crate) fn print_sol_2(file: &str) {
    print!("res : {}", eval_file_2(file));
}

#[cfg(test)]
mod tests {
    use super::{eval_file, eval_file_2};

    fn data() -> &'static str {
        r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#
    }
    #[test]
    fn test_0() {
        assert_eq!(19114, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(167409079868000, eval_file_2(data()));
    }
}
