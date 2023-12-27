use std::str::FromStr;

use crate::{
    day_19::{factory::Factory, part::Part, workflow::Workflow},
    utils::file_utils::read_lines,
};

use super::{part_range::PartRange, rule::Destination};

pub fn solve() {
    let filename = "src/day_19/sample.txt";

    let lines = read_lines(filename);
    let lines = lines.iter().map(|l| l.as_str()).collect::<Vec<&str>>();

    let (workflows, parts) = split_lines(lines);

    let workflows = workflows
        .iter()
        .map(|l| Workflow::from_str(*l).unwrap())
        .collect::<Vec<Workflow>>();

    let parts = parts
        .iter()
        .map(|l| Part::from_str(*l).unwrap())
        .collect::<Vec<Part>>();

    let factory = Factory::new(workflows);
    let sum_accepted = sum_accepted(&factory, &parts);
    let count = count_ranges(&factory);

    println!("Day 19");
    println!("The sum of qualities of accepted parts is {sum_accepted}.");
    println!("The number of acceptable parts is {count}.");
}

fn split_lines(lines: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut workflows = vec![];
    let mut parts = vec![];

    let mut is_workflow = true;

    for l in lines {
        if l.trim().is_empty() {
            is_workflow = false;
        } else if is_workflow {
            workflows.push(l);
        } else {
            parts.push(l);
        }
    }

    return (workflows, parts);
}

fn sum_accepted(factory: &Factory, parts: &Vec<Part>) -> u32 {
    let accepted = parts
        .iter()
        .filter(|p| factory.process(p) == &Destination::Accepted)
        .collect::<Vec<&Part>>();

    return accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum();
}

fn count_ranges(factory: &Factory) -> u32 {
    let accepted = factory.process_range(&PartRange::full());

    return accepted.iter().map(|a| a.count()).sum();
}
