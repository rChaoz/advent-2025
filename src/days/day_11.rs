use crate::RunType::Examples;
use crate::{Context, DayInfo};
use kust::ScopeFunctions;
use std::collections::HashMap;

pub const INFO: DayInfo = DayInfo {
    name: "Reactor",
    run,
    example: "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
};

const EXAMPLE2: &'static str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut adj = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(':');
        let source = parts.next().unwrap();

        let targets = parts.next().unwrap().trim().split_whitespace();
        adj.insert(source, Vec::new());
        let source_adj = adj.get_mut(source).unwrap();
        for target in targets {
            source_adj.push(target);
        }
    }
    adj
}

fn count_paths(adj: &HashMap<&str, Vec<&str>>, start: &str, end: &str, must_visit: &[&str]) -> u64 {
    fn bfs<'s>(
        adj: &HashMap<&'s str, Vec<&'s str>>,
        vis: &mut HashMap<&'s str, HashMap<Vec<&'s str>, u64>>,
        current: &'s str,
        target: &'s str,
        must_visit: &Vec<&'s str>,
    ) -> u64 {
        if let Some(inner) = vis.get(current)
            && let Some(&known_count) = inner.get(must_visit)
        {
            return known_count;
        }
        if current == target {
            return if must_visit.is_empty() { 1 } else { 0 };
        }
        let must_visit = must_visit
            .iter()
            .copied()
            .filter(|&node| node != current)
            .collect::<Vec<_>>();
        adj[current]
            .iter()
            .map(|&node| bfs(adj, vis, node, target, &must_visit))
            .sum::<u64>()
            .also(|&count| {
                vis.entry(current)
                    .or_insert_with(|| HashMap::new())
                    .insert(must_visit, count)
            })
    }

    bfs(adj, &mut HashMap::new(), start, end, &Vec::from(must_visit))
}

fn run(context: &mut Context) {
    let mut adj = parse(context.input);
    context.result(count_paths(&adj, "you", "out", &[]));
    if context.run_type == Examples {
        adj = parse(EXAMPLE2);
    }
    context.result(count_paths(&adj, "svr", "out", &["dac", "fft"]));
}
