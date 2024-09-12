use std::{cmp::min, collections::VecDeque};

use crate::{Term, Var};

use super::weak_cnf::{WeakCnf, WeakCnfRule};
use super::{Cnf, CnfRule};

pub fn eliminate_unit_rules<T: Term>(weak_cnf: WeakCnf<T>) -> Cnf<T> {
    let mut graph: Vec<Vec<usize>> = vec![vec![]; weak_cnf.nullable.len()];

    for rule in &weak_cnf.rules {
        if let WeakCnfRule::Unit(first, second) = rule {
            graph[first.0].push(second.0);
        }
    }

    let sccs = StronglyConnectedComponents::of(&graph);
    let mut vars_map = vec![Var(0); graph.len()];
    for (index, scc) in sccs.iter().enumerate() {
        for u in scc {
            vars_map[*u] = Var(index)
        }
    }

    let reversed_quotient_graph = get_reversed_quotient_graph(&graph, sccs.len(), &vars_map);

    let nonunit_rules_at_each_scc =
        get_nonunit_rules_at_each_scc(weak_cnf.rules, sccs.len(), &vars_map);

    let rules = replace_unit_rules_at_each_scc(
        sccs.len(),
        reversed_quotient_graph,
        nonunit_rules_at_each_scc,
    );

    Cnf {
        nullable: weak_cnf.nullable,
        rules,
        vars_map,
    }
}

fn get_reversed_quotient_graph(
    graph: &Vec<Vec<usize>>,
    n_sccs: usize,
    vars_map: &Vec<Var>,
) -> Vec<Vec<usize>> {
    let mut reversed_quotient_graph: Vec<Vec<usize>> = vec![vec![]; n_sccs];

    for (u, adj) in graph.iter().enumerate() {
        for v in adj.iter().cloned() {
            reversed_quotient_graph[vars_map[v].0].push(vars_map[u].0);
        }
    }

    reversed_quotient_graph
}

fn get_nonunit_rules_at_each_scc<T: Term>(
    weak_cnf_rules: Vec<WeakCnfRule<T>>,
    n_sccs: usize,
    vars_map: &Vec<Var>,
) -> Vec<Vec<CnfRule<T>>> {
    let mut nonunit_rules: Vec<Vec<CnfRule<T>>> = vec![vec![]; n_sccs];

    for rule in weak_cnf_rules {
        match rule {
            WeakCnfRule::Terminal(var, term) => {
                nonunit_rules[vars_map[var.0].0].push(CnfRule::Terminal(vars_map[var.0], term));
            }
            WeakCnfRule::Binary(first, second, third) => {
                nonunit_rules[vars_map[first.0].0].push(CnfRule::Binary(
                    vars_map[first.0],
                    vars_map[second.0],
                    vars_map[third.0],
                ));
            }
            _ => {}
        };
    }

    nonunit_rules
}

fn replace_unit_rules_at_each_scc<T: Term>(
    n_sccs: usize,
    mut reversed_quotient_graph: Vec<Vec<usize>>,
    mut nonunit_rules: Vec<Vec<CnfRule<T>>>,
) -> Vec<CnfRule<T>> {
    // Degree = Indegree in reversed quotient graph
    let mut degree = vec![0; n_sccs];
    for (_, adj_list) in reversed_quotient_graph.iter_mut().enumerate() {
        for v in adj_list.iter() {
            degree[*v] += 1;
        }
        adj_list.sort();
        adj_list.dedup();
    }

    let mut queue = VecDeque::<usize>::new();
    for u in 0..n_sccs {
        if degree[u] == 0 {
            queue.push_back(u);
        }
    }

    while let Some(u) = queue.pop_front() {
        nonunit_rules[u].sort();
        nonunit_rules[u].dedup();
        for v in reversed_quotient_graph[u].iter().cloned() {
            degree[v] -= 1;
            if degree[v] == 0 {
                queue.push_back(v);
            }
            let nonunit_rules_at_u = nonunit_rules[u].clone();
            for mut rule in nonunit_rules_at_u.into_iter() {
                match &mut rule {
                    CnfRule::Terminal(var, _) => {
                        *var = Var(v);
                    }
                    CnfRule::Binary(first, _, _) => {
                        *first = Var(v);
                    }
                }
                nonunit_rules[v].push(rule);
            }
        }
    }

    let mut all_nonunit_rules = Vec::<CnfRule<T>>::new();

    for nonunit_rules in nonunit_rules.into_iter() {
        all_nonunit_rules.extend(nonunit_rules.into_iter());
    }

    all_nonunit_rules
}

struct StronglyConnectedComponents<'a> {
    graph: &'a Vec<Vec<usize>>,
    timer: isize,
    tin: Vec<isize>,
    low: Vec<isize>,
    all: Vec<usize>,
    sccs: Vec<Vec<usize>>,
}

impl<'a> StronglyConnectedComponents<'a> {
    pub fn of(graph: &'a Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let n = graph.len();
        let mut solver = Self {
            graph,
            timer: 0,
            tin: vec![-1; n],
            low: vec![0; n],
            all: Vec::with_capacity(n),
            sccs: vec![],
        };

        for u in 0..n {
            if solver.tin[u] == -1 {
                solver.dfs(u);
            }
        }

        solver.sccs
    }

    fn dfs(&mut self, u: usize) {
        self.low[u] = self.timer;
        self.tin[u] = self.timer;
        self.timer += 1;

        self.all.push(u);

        for v in self.graph[u].iter() {
            if self.tin[*v] == -1 {
                self.dfs(*v);
            }
            self.low[u] = min(self.low[u], self.low[*v]);
        }
        if self.low[u] == self.tin[u] {
            let sz = self.all.iter().rev().position(|&x| x == u).unwrap();
            self.sccs.push(Vec::with_capacity(sz + 1));
            for _ in 0..sz + 1 {
                let v = self.all.pop().unwrap();
                self.low[v] = self.graph.len() as isize;
                self.sccs.last_mut().unwrap().push(v);
            }
        }
    }
}
