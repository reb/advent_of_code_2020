extern crate itertools;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate num_derive;
extern crate num_traits;
extern crate petgraph;

use std::collections::HashMap;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let modules = create_modules();
    while let Some(arg) = args.next() {
        modules.run(&arg);
    }
}

type Run = fn();

struct Modules {
    runners: HashMap<String, Run>,
}

impl Modules {
    fn add_module(&mut self, name: String, func: Run) {
        self.runners.insert(name, func);
    }

    fn run(&self, name: &String) {
        let runner = self.runners.get(name).unwrap();
        runner();
    }
}

macro_rules! modules {
    ($($mod:ident,)*) => {
        $( mod $mod; )*

        fn create_modules() -> Modules {
            let mut modules = Modules { runners: HashMap::new() };
            $( modules.add_module(stringify!($mod).to_string(), $mod::run); )*
            modules
        }
    };
}

modules![
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, day_11, day_12,
    day_13, day_14, day_15, day_16, day_17, day_18, day_19,
];
