use std::collections::{HashMap, HashSet};
use crate::formatting::traits::FormatterRule;
use crate::formatting::metadata::{RuleCategory, RuleId};

pub struct ExecutionPlanner;

#[derive(Debug)]
pub enum PlannerError {
    MissingDependency { rule: String, depends_on: String },
    CrossCategoryDependency { rule: String, depends_on: String },
    DuplicateRuleId(String),
    CircularDependency(String),
    SelfDependency(String),
}

impl std::fmt::Display for PlannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlannerError::MissingDependency { rule, depends_on } => write!(f, "Rule '{}' depends on missing rule '{}'", rule, depends_on),
            PlannerError::CrossCategoryDependency { rule, depends_on } => write!(f, "Cross-category dependency: '{}' depends on '{}'", rule, depends_on),
            PlannerError::DuplicateRuleId(id) => write!(f, "Duplicate rule ID: '{}'", id),
            PlannerError::CircularDependency(id) => write!(f, "Circular dependency detected involving: '{}'", id),
            PlannerError::SelfDependency(id) => write!(f, "Rule '{}' depends on itself", id),
        }
    }
}
impl std::error::Error for PlannerError {}

impl ExecutionPlanner {
    pub fn plan(
        mut rules: Vec<Box<dyn FormatterRule>>,
        passes: &[RuleCategory],
    ) -> Result<Vec<Box<dyn FormatterRule>>, PlannerError> {
        let mut categorized_rules: HashMap<RuleCategory, Vec<Box<dyn FormatterRule>>> = HashMap::new();
        
        let mut rule_category_map = HashMap::new();
        let mut rule_names = HashSet::new();
        
        for rule in &rules {
            let meta = rule.metadata();
            if !rule_names.insert(meta.id.0) {
                return Err(PlannerError::DuplicateRuleId(meta.id.0.to_string()));
            }
            rule_category_map.insert(meta.id.0, meta.category);
        }
        
        // Validation
        for rule in &rules {
            let meta = rule.metadata();
            for dep in meta.depends_on {
                if dep.0 == meta.id.0 {
                    return Err(PlannerError::SelfDependency(meta.id.0.to_string()));
                }
                if let Some(dep_category) = rule_category_map.get(dep.0) {
                    if *dep_category != meta.category {
                        return Err(PlannerError::CrossCategoryDependency {
                            rule: meta.id.0.to_string(),
                            depends_on: dep.0.to_string(),
                        });
                    }
                } else {
                    return Err(PlannerError::MissingDependency {
                        rule: meta.id.0.to_string(),
                        depends_on: dep.0.to_string(),
                    });
                }
            }
        }
        
        // Group by category
        for rule in rules.into_iter() {
            let cat = rule.metadata().category;
            categorized_rules.entry(cat).or_default().push(rule);
        }
        
        let mut final_ordered_rules = Vec::new();
        
        // Topo sort per category
        for category in passes {
            if let Some(mut category_rules) = categorized_rules.remove(category) {
                // Sort by priority first as a tie breaker
                category_rules.sort_by(|a, b| b.metadata().priority.cmp(&a.metadata().priority));
                
                let mut in_degree: HashMap<&str, usize> = HashMap::new();
                let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
                let mut rule_map: HashMap<&str, Box<dyn FormatterRule>> = HashMap::new();
                
                for rule in category_rules {
                    let id = rule.metadata().id.0;
                    in_degree.insert(id, 0);
                    rule_map.insert(id, rule);
                }
                
                for rule in rule_map.values() {
                    let id = rule.metadata().id.0;
                    for dep in rule.metadata().depends_on {
                        graph.entry(dep.0).or_default().push(id);
                        *in_degree.get_mut(id).unwrap() += 1;
                    }
                }
                
                // Keep the queue ordered by priority (using the fact that rule_map isn't ordered, but we can sort the initial 0-degree nodes)
                let mut zero_in_degree = Vec::new();
                for (id, &deg) in &in_degree {
                    if deg == 0 {
                        zero_in_degree.push(*id);
                    }
                }
                
                // Sort zero_in_degree by priority descending
                zero_in_degree.sort_by(|a, b| {
                    let pa = rule_map.get(a).unwrap().metadata().priority;
                    let pb = rule_map.get(b).unwrap().metadata().priority;
                    pb.cmp(&pa)
                });
                
                let mut sorted_ids = Vec::new();
                
                while !zero_in_degree.is_empty() {
                    // Pop from the front to preserve priority order (highest first)
                    // Wait, we want to pop the highest priority. Since we sorted descending, the first element has the highest priority.
                    // But popping from a Vec takes from the back. So we should sort ascending and pop, or just use a stable way.
                    // Let's remove from index 0. (O(n) but N is small).
                    let current = zero_in_degree.remove(0);
                    sorted_ids.push(current);
                    
                    if let Some(neighbors) = graph.get(current) {
                        for &neighbor in neighbors {
                            let deg = in_degree.get_mut(neighbor).unwrap();
                            *deg -= 1;
                            if *deg == 0 {
                                zero_in_degree.push(neighbor);
                            }
                        }
                    }
                    
                    // Re-sort zero_in_degree by priority descending
                    zero_in_degree.sort_by(|a, b| {
                        let pa = rule_map.get(a).unwrap().metadata().priority;
                        let pb = rule_map.get(b).unwrap().metadata().priority;
                        pb.cmp(&pa)
                    });
                }
                
                if sorted_ids.len() != rule_map.len() {
                    // Find a rule that is missing
                    let missing = rule_map.keys().find(|k| !sorted_ids.contains(k)).unwrap();
                    return Err(PlannerError::CircularDependency(missing.to_string()));
                }
                
                for id in sorted_ids {
                    final_ordered_rules.push(rule_map.remove(id).unwrap());
                }
            }
        }
        
        Ok(final_ordered_rules)
    }
}
