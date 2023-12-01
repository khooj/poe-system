use timely::dataflow::operators::{Exchange, Input, Inspect, Probe, ToStream};
use timely::dataflow::InputHandle;

fn main() {
    timely::execute_from_args(std::env::args(), |worker| {
        let index = worker.index();
        let mut input = InputHandle::new();

        let probe = worker.dataflow(|scope| {
            scope
                .input_from(&mut input)
                .exchange(|x| *x)
                .inspect(move |x| println!("worker: {}:\thello {}", index, x))
                .probe()
        });

        for round in 0..10 {
            if index == 0 {
                input.send(round);
            }
            input.advance_to(round + 1);
            while probe.less_than(input.time()) {
                worker.step();
            }
        }
    })
    .unwrap();
}
