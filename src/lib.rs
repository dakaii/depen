use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct DepName(String);

impl DepName {
    pub fn new(name: String) -> DepName {
        DepName(name)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Dep {
    pub name: DepName,
    // I assumed the attribute below should be added even though it was not shown in the assignment prompt.
    pub transitive_deps: Vec<Dep>,
}

impl Dep {
    pub fn deps(&self) -> Vec<Dep> {
        self.transitive_deps.to_owned()
    }
}

pub fn resolve_dependency_order(deps: Vec<Dep>) -> Vec<DepName> {
    let mut dependency_graph: HashMap<DepName, Vec<Dep>> = HashMap::new();
    for dep in &deps {
        dependency_graph.insert(dep.name.to_owned(), dep.deps());
    }
    let mut resolved = HashSet::new();
    let mut ordered_deps = Vec::new();
    for dep in &deps {
        if resolved.contains(&dep.name) {
            continue;
        }
        // the stack below is the list of deps that are currently being checked
        let mut stack = Vec::from([dep.name.to_owned()]);
        while let Some(curr) = stack.last().cloned() {
            let transitive_deps = match dependency_graph.get(&curr) {
                Some(d) => d.to_owned(),
                None => Vec::new(),
            };
            let mut is_installable_without_deps = true;
            for transitive_dep in transitive_deps {
                if resolved.contains(&transitive_dep.name) {
                    continue;
                } else if stack.contains(&transitive_dep.name) {
                    panic!("cyclic dependency error!: caused by {:#?}", curr);
                }
                stack.push(transitive_dep.name.to_owned());
                dependency_graph
                    .entry(transitive_dep.name.to_owned())
                    .or_insert(transitive_dep.deps());
                is_installable_without_deps = false;
                break;
            }
            if is_installable_without_deps {
                if let Some(cleared_dep) = stack.pop() {
                    ordered_deps.push(cleared_dep.to_owned());
                    resolved.insert(cleared_dep);
                }
            }
        }
    }
    ordered_deps
}
