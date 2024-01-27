use std::collections::{VecDeque, HashSet};
use std::fmt::Debug;
use std::ops::Deref;
use std::{collections::HashMap, fs};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum PulseType {
    LOW,
    HIGH,
}

trait Module: std::fmt::Debug {
    fn receive(&mut self, pulse: PulseType, input: String) -> Option<PulseType>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FlipFlopModule {
    state: bool,
}

impl Module for FlipFlopModule {
    fn receive(&mut self, pulse: PulseType, input: String) -> Option<PulseType> {
        if pulse == PulseType::LOW {
            self.state = !self.state;
            let pulse = if self.state {
                PulseType::HIGH
            } else {
                PulseType::LOW
            };
            Some(pulse)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConjunctionModule {
    inputs: HashMap<String, PulseType>,
}

impl Module for ConjunctionModule {
    fn receive(&mut self, pulse: PulseType, input: String) -> Option<PulseType> {
        *(self.inputs.get_mut(&input).unwrap()) = pulse;
        if self.inputs.values().all(|x| *x == PulseType::HIGH) {
            Some(PulseType::LOW)
        } else {
            Some(PulseType::HIGH)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BroadcasterModule {}

impl Module for BroadcasterModule {
    fn receive(&mut self, pulse: PulseType, input: String) -> Option<PulseType> {
        Some(pulse)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NoOpModule {}

impl Module for NoOpModule {
    fn receive(&mut self, pulse: PulseType, input: String) -> Option<PulseType> {
        None
    }
}

#[derive(Debug)]
struct ModuleDef {
    module: Box<dyn Module>,
    outputs: Vec<String>,
}

fn parse(lines: &Vec<&str>) -> HashMap<String, ModuleDef> {
    let mut map = HashMap::new();

    let mut module_inputs: HashMap<String, Vec<String>> = HashMap::new();
    let mut module_outputs: HashMap<String, Vec<String>> = HashMap::new();
    let mut all_modules: HashSet<String> = HashSet::new();

    for line in lines {
        let spl = line.split(" -> ").collect::<Vec<_>>();
        let name = spl[0];
        let name_string = if name.starts_with("%") || name.starts_with("&") {
            String::from(&name[1..])
        } else {
            String::from(name)
        };
        let outputs = spl[1]
            .split(", ")
            .map(|x| String::from(x))
            .collect::<Vec<_>>();
        for output in &outputs {
            module_inputs
                .entry(output.clone())
                .or_insert(Vec::new())
                .push(name_string.clone());
            all_modules.insert(output.clone());
        }
        all_modules.insert(name_string.clone());
        module_outputs.insert(name_string.clone(), outputs.clone());
    }

    for line in lines {
        let spl = line.split(" -> ").collect::<Vec<_>>();
        let name = spl[0];
        let name_string = if name.starts_with("%") || name.starts_with("&") {
            String::from(&name[1..])
        } else {
            String::from(name)
        };
        let module: Box<dyn Module> = if name.starts_with("%") {
            // flip flop
            Box::new(FlipFlopModule { state: false })
        } else if name.starts_with("&") {
            // conjunction
            let mut inputs: HashMap<String, PulseType> = HashMap::new();
            for i in &module_inputs[&name_string] {
                inputs.insert(i.clone(), PulseType::LOW);
            }
            Box::new(ConjunctionModule { inputs })
        } else if name == "broadcaster" {
            Box::new(BroadcasterModule {})
        } else {
            Box::new(NoOpModule {})
        };
        map.insert(name_string.clone(), ModuleDef { module, outputs: module_outputs[&name_string].clone() });
    }

    // additional modules (output)
    for module in &all_modules {
        if !map.contains_key(module) {
            map.insert(module.clone(), ModuleDef {
                module: Box::new(NoOpModule {
                }),
                outputs: Vec::new()
            });
        }
    }

    map
}

fn part1(modules: &mut HashMap<String, ModuleDef>, count: usize) -> u64 {
    let mut high_counts = 0;
    let mut low_counts = 0;
    for _ in 0..count {
        let mut q = VecDeque::new();
        q.push_back((
            String::from("button"),
            String::from("broadcaster"),
            PulseType::LOW,
        ));

        while !q.is_empty() {
            let (from, to, pulse) = q.pop_front().unwrap();
            if pulse == PulseType::HIGH {
                high_counts += 1;
            } else {
                low_counts += 1;
            }
            // println!("to {:?}", to);
            let def = modules.get_mut(&to).unwrap();
            let res = def.module.receive(pulse, from);
            if let Some(output_pulse) = res {
                for output in &def.outputs {
                    q.push_back((to.clone(), output.clone(), output_pulse));
                }
            }
        }
    }
    // println!("{} {}", low_counts, high_counts);
    low_counts * high_counts
}

fn main() {
    let content = fs::read_to_string("inputs/day20.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let mut modules = parse(&lines);

    // part 1
    println!("{}", part1(&mut modules, 1000));
}
