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
            if resolved.contains(&curr) {
                stack.pop();
                continue;
            }

            let transitive_deps = match dependency_graph.get(&curr) {
                Some(d) => d.to_owned(),
                None => Vec::new(),
            };
            let mut is_installable_without_deps = true;
            for transitive_dep in transitive_deps {
                // println!("start");
                // println!("curr {:#?}", curr);
                // println!("{:#?}", transitive_dep.name);
                // println!("end");
                if resolved.contains(&transitive_dep.name) {
                    continue;
                } else if stack.contains(&transitive_dep.name) {
                    println!("{:#?}", transitive_dep.name);
                    println!("curr {:#?}", curr);
                    panic!("cyclic dependency error!");
                }
                stack.push(transitive_dep.name.to_owned());
                is_installable_without_deps = false;
                dependency_graph
                    .entry(transitive_dep.name.to_owned())
                    .or_insert(transitive_dep.deps());
                break;
            }
            if is_installable_without_deps {
                let cleared_dep = stack.pop().unwrap();
                ordered_deps.push(cleared_dep.to_owned());
                resolved.insert(cleared_dep);
            }
        }
    }
    ordered_deps
}

// fn main() {
//     let super_super_deep = Dep {
//         name: DepName("super_super_deep".to_owned()),
//         transitive_deps: Vec::new(),
//     };
//     let super_deep = Dep {
//         name: DepName("super_deep".to_owned()),
//         transitive_deps: Vec::from([super_super_deep.to_owned()]),
//     };
//     let deep_dep = Dep {
//         name: DepName("deep_dep".to_owned()),
//         transitive_deps: Vec::from([super_deep.to_owned()]),
//     };
//     let another_deep_dep = Dep {
//         name: DepName("another_dep".to_owned()),
//         transitive_deps: Vec::from([deep_dep.to_owned()]),
//     };
//     let first_dep = Dep {
//         name: DepName("first_dep".to_owned()),
//         transitive_deps: Vec::from([another_deep_dep.to_owned(), deep_dep]),
//     };
//     let third_dep = Dep {
//         name: DepName("third_dep".to_owned()),
//         transitive_deps: Vec::from([first_dep.to_owned()]),
//     };
//     let second_dep_dep = Dep {
//         name: DepName("second_dep_dep".to_owned()),
//         transitive_deps: Vec::from([third_dep.to_owned(), another_deep_dep]),
//     };
//     let second_dep = Dep {
//         name: DepName("second_dep".to_owned()),
//         transitive_deps: Vec::from([third_dep.to_owned(), second_dep_dep.to_owned()]),
//     };

//     let mut vec = Vec::new();
//     vec.push(first_dep);
//     vec.push(second_dep);
//     vec.push(third_dep);
//     let deps = resolve_dependency_order(vec);
//     println!("{:#?}", deps);
// }
