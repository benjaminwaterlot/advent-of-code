pub mod parser;

type Item = isize;

#[derive(Debug)]
enum Operation {
    Add(isize),
    Multiply(isize),
    Square,
}
type Target = (isize, isize);

#[derive(Debug)]
pub struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: isize,
    target: Target,
}

impl Monkey {
    fn drop_item(&mut self) -> Option<isize> {
        self.items.pop()
    }

    fn get_item(&mut self, item: isize) {
        self.items.push(item);
    }
}

pub struct Game {
    monkeys: Vec<Monkey>,
    round: isize,
}

impl Game {
    fn do_round(&mut self) {
        let len = self.monkeys.len();

        for i in 0..len {
            for item_index in 0..self.monkeys[i].items.len() {
                // let dropped_item = self.monkeys[i].drop_item().unwrap();
                self.monkeys.last().unwrap().get_item(123);
            }
        }

        // for monkey in &mut self.monkeys {
        //     while monkey.items.len() > 0 {
        //         let dropped_item = monkey.drop_item();
        //         match dropped_item {
        //             None => continue,
        //             Some(x) => self.monkeys.last().unwrap().get_item(x),
        //         }
        //     }

        //     // for item in &monkey.items {
        //     //     // let dropped_item_2 = monkey.drop_item();
        //     //     println!("{item}");
        //     // }
        // }
    }
}

fn main() {
    let mut monkeys = parser::parse_input();

    println!("parsed monkeys: {monkeys:#?}");

    let mut game = Game { monkeys, round: 0 };

    game.do_round();
}
