use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct SkillDependency {
    pub name: String,
    pub version_req: String,
}

pub struct DependencyGraph {
    edges: HashMap<String, Vec<SkillDependency>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self { edges: HashMap::new() }
    }

    pub fn add_dependency(&mut self, skill: &str, depends_on: &str, version_req: &str) {
        self.edges.entry(skill.to_string())
            .or_default()
            .push(SkillDependency {
                name: depends_on.to_string(),
                version_req: version_req.to_string(),
            });
    }

    pub fn get_dependencies(&self, skill: &str) -> Vec<&SkillDependency> {
        self.edges.get(skill)
            .map(|d| d.iter().collect())
            .unwrap_or_default()
    }

    pub fn get_dependents(&self, skill: &str) -> Vec<String> {
        self.edges.iter()
            .filter(|(_, deps)| deps.iter().any(|d| d.name == skill))
            .map(|(k, _)| k.clone())
            .collect()
    }

    pub fn has_cycle(&self) -> Option<Vec<String>> {
        let mut visited = HashSet::new();
        let mut in_stack = HashSet::new();
        let mut path = Vec::new();

        for node in self.edges.keys() {
            if !visited.contains(node) {
                if let Some(cycle) = self.dfs_cycle(node, &mut visited, &mut in_stack, &mut path) {
                    return Some(cycle);
                }
            }
        }
        None
    }

    fn dfs_cycle<'a>(
        &'a self,
        node: &str,
        visited: &mut HashSet<String>,
        in_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(node.to_string());
        in_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(deps) = self.edges.get(node) {
            for dep in deps {
                if !visited.contains(&dep.name) {
                    if let Some(cycle) = self.dfs_cycle(&dep.name, visited, in_stack, path) {
                        return Some(cycle);
                    }
                } else if in_stack.contains(&dep.name) {
                    let idx = path.iter().position(|p| p == &dep.name).unwrap();
                    let cycle = path[idx..].to_vec();
                    return Some(cycle);
                }
            }
        }

        path.pop();
        in_stack.remove(node);
        None
    }

    pub fn resolve_order(&self) -> Result<Vec<String>, String> {
        if let Some(cycle) = self.has_cycle() {
            return Err(format!("Dongusel bagimlilik: {}", cycle.join(" -> ")));
        }

        let mut visited = HashSet::new();
        let mut order = Vec::new();

        for node in self.edges.keys() {
            if !visited.contains(node) {
                self.topological_sort(node, &mut visited, &mut order);
            }
        }
        Ok(order)
    }

    fn topological_sort(&self, node: &str, visited: &mut HashSet<String>, order: &mut Vec<String>) {
        visited.insert(node.to_string());
        if let Some(deps) = self.edges.get(node) {
            for dep in deps {
                if !visited.contains(&dep.name) {
                    self.topological_sort(&dep.name, visited, order);
                }
            }
        }
        order.push(node.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph_no_cycle() {
        let g = DependencyGraph::new();
        assert!(g.has_cycle().is_none());
    }

    #[test]
    fn simple_dependency() {
        let mut g = DependencyGraph::new();
        g.add_dependency("Skill_A", "Skill_B", "1.0.0");
        let deps = g.get_dependencies("Skill_A");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "Skill_B");
    }

    #[test]
    fn detect_cycle() {
        let mut g = DependencyGraph::new();
        g.add_dependency("A", "B", "1.0");
        g.add_dependency("B", "C", "1.0");
        g.add_dependency("C", "A", "1.0");
        assert!(g.has_cycle().is_some());
    }

    #[test]
    fn no_cycle_linear() {
        let mut g = DependencyGraph::new();
        g.add_dependency("A", "B", "1.0");
        g.add_dependency("B", "C", "1.0");
        assert!(g.has_cycle().is_none());
    }

    #[test]
    fn resolve_order_linear() {
        let mut g = DependencyGraph::new();
        g.add_dependency("A", "B", "1.0");
        g.add_dependency("B", "C", "1.0");
        let order = g.resolve_order().unwrap();
        // C should come before B, B before A
        let c_pos = order.iter().position(|s| s == "C").unwrap();
        let b_pos = order.iter().position(|s| s == "B").unwrap();
        let a_pos = order.iter().position(|s| s == "A").unwrap();
        assert!(c_pos < b_pos);
        assert!(b_pos < a_pos);
    }

    #[test]
    fn resolve_order_with_cycle_returns_error() {
        let mut g = DependencyGraph::new();
        g.add_dependency("A", "B", "1.0");
        g.add_dependency("B", "A", "1.0");
        assert!(g.resolve_order().is_err());
    }

    #[test]
    fn get_dependents_finds_reverse() {
        let mut g = DependencyGraph::new();
        g.add_dependency("A", "Core", "1.0");
        g.add_dependency("B", "Core", "1.0");
        let deps = g.get_dependents("Core");
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&"A".to_string()));
        assert!(deps.contains(&"B".to_string()));
    }

    #[test]
    fn multiple_dependencies() {
        let mut g = DependencyGraph::new();
        g.add_dependency("App", "Lib", "2.0");
        g.add_dependency("App", "Utils", "1.5");
        let deps = g.get_dependencies("App");
        assert_eq!(deps.len(), 2);
    }
}
