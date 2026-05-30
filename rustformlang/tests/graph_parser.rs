use rustformlang::cfg::cfg::{CFG, TerminalGraphEdge};
use rustformlang::cfg::production::{Production, Symbol};
use rustformlang::cfg::terminal::Terminal;
use rustformlang::cfg::variable::Variable;
use std::time::Duration;

fn tiny_ab_cfg() -> CFG {
    let s = Variable::new("S");
    let a_nt = Variable::new("A");
    let b_nt = Variable::new("B");
    CFG::from_start_and_productions(
        s.clone(),
        vec![
            Production::new(s.clone(), vec![Symbol::V(a_nt.clone()), Symbol::V(b_nt.clone())]),
            Production::new(a_nt.clone(), vec![Symbol::T(Terminal::new("a"))]),
            Production::new(b_nt.clone(), vec![Symbol::T(Terminal::new("b"))]),
        ],
    )
    .to_normal_form()
}

#[test]
fn graph_parser_accepts_simple_path() {
    let cfg = tiny_ab_cfg();
    let terminal_map = cfg.get_terminal_map();
    let a = *terminal_map.get(&Terminal::new("a")).unwrap();
    let b = *terminal_map.get(&Terminal::new("b")).unwrap();
    let graph = vec![
        TerminalGraphEdge { src: 0, dst: 1, terminal: Some(a) },
        TerminalGraphEdge { src: 1, dst: 2, terminal: Some(b) },
    ];
    assert!(!cfg.is_graph_intersection_empty(3, 0, &[2], &graph, Some(Duration::from_secs(1))));
}

#[test]
fn graph_parser_rejects_bad_path() {
    let cfg = tiny_ab_cfg();
    let terminal_map = cfg.get_terminal_map();
    let a = *terminal_map.get(&Terminal::new("a")).unwrap();
    let graph = vec![
        TerminalGraphEdge { src: 0, dst: 1, terminal: Some(a) },
        TerminalGraphEdge { src: 1, dst: 2, terminal: Some(a) },
    ];
    assert!(cfg.is_graph_intersection_empty(3, 0, &[2], &graph, Some(Duration::from_secs(1))));
}

#[test]
fn graph_parser_handles_epsilon_edges() {
    let cfg = tiny_ab_cfg();
    let terminal_map = cfg.get_terminal_map();
    let a = *terminal_map.get(&Terminal::new("a")).unwrap();
    let b = *terminal_map.get(&Terminal::new("b")).unwrap();
    let graph = vec![
        TerminalGraphEdge { src: 0, dst: 1, terminal: Some(a) },
        TerminalGraphEdge { src: 1, dst: 2, terminal: None },
        TerminalGraphEdge { src: 2, dst: 3, terminal: Some(b) },
    ];
    assert!(!cfg.is_graph_intersection_empty(4, 0, &[3], &graph, Some(Duration::from_secs(1))));
}
