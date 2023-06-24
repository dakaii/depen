use depen::{resolve_dependency_order, Dep, DepName};

#[test]
fn it_resolves_dependency_order_without_transitive_deps() {
    let example_dep = Dep {
        name: DepName::new("example_dep".to_owned()),
        transitive_deps: Vec::new(),
    };
    let another_dep = Dep {
        name: DepName::new("another_dep".to_owned()),
        transitive_deps: Vec::new(),
    };

    let mut vec = Vec::new();
    vec.push(example_dep.to_owned());
    vec.push(another_dep.to_owned());
    let deps = resolve_dependency_order(vec);

    let mut correct_answer = Vec::new();
    correct_answer.push(example_dep.name);
    correct_answer.push(another_dep.name);
    assert_eq!(correct_answer, deps);
}

#[test]
fn it_resolves_complex_dependency_order() {
    let super_super_deep = Dep {
        name: DepName::new("super_super_deep".to_owned()),
        transitive_deps: Vec::new(),
    };
    let super_deep = Dep {
        name: DepName::new("super_deep".to_owned()),
        transitive_deps: Vec::from([super_super_deep.to_owned()]),
    };
    let example_dep = Dep {
        name: DepName::new("example_dep".to_owned()),
        transitive_deps: Vec::from([super_deep.to_owned(), super_super_deep.to_owned()]),
    };
    let another_dep = Dep {
        name: DepName::new("another_dep".to_owned()),
        transitive_deps: Vec::from([example_dep.to_owned()]),
    };
    let one_more_dep = Dep {
        name: DepName::new("one_more_dep".to_owned()),
        transitive_deps: Vec::from([another_dep.to_owned(), super_deep.to_owned()]),
    };
    let amazing_dep = Dep {
        name: DepName::new("amazing_dep".to_owned()),
        transitive_deps: Vec::from([another_dep.to_owned(), one_more_dep.to_owned()]),
    };

    let mut vec = Vec::new();
    vec.push(another_dep.to_owned());
    vec.push(amazing_dep.to_owned());
    vec.push(example_dep.to_owned());
    let deps = resolve_dependency_order(vec);

    let mut correct_answer = Vec::new();
    correct_answer.push(super_super_deep.name);
    correct_answer.push(super_deep.name);
    correct_answer.push(example_dep.name);
    correct_answer.push(another_dep.name);
    correct_answer.push(one_more_dep.name);
    correct_answer.push(amazing_dep.name);
    assert_eq!(correct_answer, deps);
}

#[test]
#[should_panic(expected = "cyclic dependency error!")]
fn it_errors_if_theres_circular_dependency() {
    let mut super_super_deep = Dep {
        name: DepName::new("super_super_deep".to_owned()),
        transitive_deps: Vec::new(),
    };
    let super_deep = Dep {
        name: DepName::new("super_deep".to_owned()),
        transitive_deps: Vec::from([super_super_deep.to_owned()]),
    };
    let example_dep = Dep {
        name: DepName::new("example_dep".to_owned()),
        transitive_deps: Vec::from([super_deep.to_owned(), super_super_deep.to_owned()]),
    };
    super_super_deep.transitive_deps = Vec::from([super_deep.to_owned()]);

    let mut vec = Vec::new();
    vec.push(super_super_deep.to_owned());
    vec.push(super_deep.to_owned());
    vec.push(example_dep.to_owned());
    let deps: Vec<DepName> = resolve_dependency_order(vec);
    println!("{:#?}", deps);

    let mut correct_answer = Vec::new();
    correct_answer.push(super_super_deep.name);
}
