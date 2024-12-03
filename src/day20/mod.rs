use std::collections::HashMap;

pub const PUZZLE_INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    Machine::parse(input).calculate()
}

pub fn part2(input: &str) -> usize {
    Machine::parse(input).initialization()
}

struct Machine<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> Machine<'a> {
    const PENULTIMATE_CONJUNCTION: &'static str = "qn";
    fn parse(input: &'a str) -> Self {
        let modules = input.split('\n').map(Module::parse).collect();
        Self { modules }
    }

    fn calculate(&mut self) -> usize {
        self.setup_conjunctions();
        let (high, low) = (0..1000).fold((0, 0), |(high, low), _| {
            let (h, l) = self.send_pulse();
            (high + h, low + l)
        });
        high * low
    }

    fn initialization(&mut self) -> usize {
        self.setup_conjunctions();
        // This is a little bit cheeky since we use knowledge of the input here
        let mut final_sources = HashMap::new();

        for i in 1..5000 {
            let mut pulses = vec![(Pulse::Low, "button", vec![Module::BROADCASTER])];

            while let Some((pulse, source, destinations)) = pulses.pop() {
                for destination in destinations {
                    if destination == Self::PENULTIMATE_CONJUNCTION && matches!(pulse, Pulse::High)
                    {
                        final_sources.entry(source).or_insert(i);
                    }
                    let module = self.modules.get_mut(destination);
                    if let Some((output, next_destinations)) =
                        module.and_then(|m| m.handle_pulse(source, pulse))
                    {
                        pulses.push((output, destination, next_destinations));
                    }
                    if final_sources.len() == 4 {
                        break;
                    }
                }
            }
        }

        final_sources
            .values()
            .copied()
            .reduce(num::integer::lcm)
            .unwrap()
    }

    fn send_pulse(&mut self) -> (usize, usize) {
        let mut high = 0;
        let mut low = 0;
        let mut pulses = vec![(Pulse::Low, "button", vec![Module::BROADCASTER])];

        while let Some((pulse, source, destinations)) = pulses.pop() {
            match pulse {
                Pulse::High => high += destinations.len(),
                Pulse::Low => low += destinations.len(),
            };
            for destination in destinations {
                let module = self.modules.get_mut(destination);
                if let Some((output, next_destinations)) =
                    module.and_then(|m| m.handle_pulse(source, pulse))
                {
                    pulses.push((output, destination, next_destinations));
                }
            }
        }
        (high, low)
    }

    fn setup_conjunctions(&mut self) {
        let conjunctions: Vec<_> = self
            .modules
            .iter()
            .filter(|(_, module)| matches!(module, Module::Conjunction { .. }))
            .map(|(conjunction, _)| {
                let inputs: Vec<_> = self
                    .modules
                    .iter()
                    .filter(|(_, module)| module.destinations().contains(conjunction))
                    .map(|(name, _)| *name)
                    .collect();
                (*conjunction, inputs)
            })
            .collect();

        conjunctions.into_iter().for_each(|(conjunction, inputs)| {
            if let Some(Module::Conjunction { state, .. }) = self.modules.get_mut(conjunction) {
                inputs.iter().for_each(|input| {
                    state.inputs.insert(input, Pulse::Low);
                });
            }
        });
    }
}

enum Module<'a> {
    Broadcaster {
        destinations: Vec<&'a str>,
    },
    Flipflop {
        state: FlipFlopState,
        destinations: Vec<&'a str>,
    },
    Conjunction {
        state: ConjunctionState<'a>,
        destinations: Vec<&'a str>,
    },
}

impl<'a> Module<'a> {
    const BROADCASTER: &'static str = "broadcaster";
    fn parse(input: &'a str) -> (&'a str, Self) {
        let (type_name, destinations) = input.split_once(" -> ").unwrap();
        let destinations = destinations.split(", ").collect();

        match type_name.chars().next().unwrap() {
            '%' => (
                &type_name[1..],
                Self::Flipflop {
                    state: Default::default(),
                    destinations,
                },
            ),
            '&' => (
                &type_name[1..],
                Self::Conjunction {
                    state: Default::default(),
                    destinations,
                },
            ),
            'b' => (Self::BROADCASTER, Self::Broadcaster { destinations }),
            _ => panic!("Unexpected input"),
        }
    }

    fn handle_pulse(&mut self, source: &'a str, input: Pulse) -> Option<(Pulse, Vec<&'a str>)> {
        match (input, self) {
            (pulse, Module::Broadcaster { ref destinations }) => {
                Some((pulse, destinations.clone()))
            }
            (Pulse::High, Module::Flipflop { .. }) => None,
            (
                Pulse::Low,
                Module::Flipflop {
                    state,
                    ref destinations,
                },
            ) => {
                let output = state.flip();
                Some((output, destinations.clone()))
            }
            (
                pulse,
                Module::Conjunction {
                    state,
                    ref destinations,
                },
            ) => {
                let output = state.output(source, pulse);
                Some((output, destinations.clone()))
            }
        }
    }

    fn destinations(&self) -> &[&'a str] {
        match self {
            Module::Broadcaster { destinations } => destinations.as_ref(),
            Module::Flipflop { destinations, .. } => destinations.as_ref(),
            Module::Conjunction { destinations, .. } => destinations.as_ref(),
        }
    }
}

#[derive(Default, Clone, Copy)]
enum FlipFlopState {
    On,
    #[default]
    Off,
}

impl FlipFlopState {
    fn flip(&mut self) -> Pulse {
        match self {
            FlipFlopState::On => {
                *self = FlipFlopState::Off;
                Pulse::Low
            }
            FlipFlopState::Off => {
                *self = FlipFlopState::On;
                Pulse::High
            }
        }
    }
}
#[derive(Default)]
struct ConjunctionState<'a> {
    inputs: HashMap<&'a str, Pulse>,
}

impl<'a> ConjunctionState<'a> {
    fn output(&mut self, source: &'a str, pulse: Pulse) -> Pulse {
        self.inputs.insert(source, pulse);
        if self.inputs.values().all(Pulse::is_high) {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn is_high(&self) -> bool {
        matches!(self, Pulse::High)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_1: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    const TEST_INPUT_2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT_1), 32000000);
        assert_eq!(part1(TEST_INPUT_2), 11687500);
    }
    #[test]
    fn test_puzzle_input_part1() {
        assert_eq!(part1(PUZZLE_INPUT), 925955316);
    }
    #[test]
    fn test_puzzle_input_part2() {
        assert_eq!(part2(PUZZLE_INPUT), 241528477694627);
    }
}
