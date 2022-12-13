use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline, space0, space1},
    combinator::{map, value},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

use std::{collections::HashMap, fs};

#[derive(Copy, Clone, Debug)]
enum Operand {
    Old,
    Value(u64),
}

#[derive(Copy, Clone, Debug)]
enum Operator {
    Multiply,
    Plus,
}

extern crate nom;

#[derive(Debug)]
struct MonkeySpec {
    number: u64,
    starting_items: Vec<u64>,
    operand_a: Operand,
    operator: Operator,
    operand_b: Operand,
    divisible_by: u64,
    true_monkey: u64,
    false_monkey: u64,
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        value(Operand::Old, tag("old")),
        map(complete::u64, Operand::Value),
    ))(input)
}

fn parse_monkey(string: &str) -> IResult<&str, MonkeySpec> {
    let monkey_number = terminated(
        preceded(tag("Monkey"), preceded(space1, complete::u64)),
        terminated(complete::char(':'), newline),
    );

    let item = preceded(space0, terminated(complete::u64, space0));
    let items = separated_list1(complete::char(','), item);
    let starting_items = preceded(space0, terminated(tag("Starting items: "), space0));
    let starting_item_values = terminated(preceded(starting_items, items), newline);

    let operator = alt((
        value(Operator::Multiply, complete::char('*')),
        value(Operator::Plus, complete::char('+')),
    ));

    let result = tuple((
        monkey_number,
        starting_item_values,
        preceded(preceded(space0, tag("Operation: new = ")), parse_operand),
        preceded(space1, terminated(operator, space1)),
        terminated(parse_operand, newline),
        terminated(
            preceded(space0, preceded(tag("Test: divisible by "), complete::u64)),
            newline,
        ),
        terminated(
            preceded(
                space0,
                preceded(tag("If true: throw to monkey "), complete::u64),
            ),
            newline,
        ),
        terminated(
            preceded(
                space0,
                preceded(tag("If false: throw to monkey "), complete::u64),
            ),
            newline,
        ),
    ));

    map(
        result,
        |(
            number,
            starting_items,
            operand_a,
            operator,
            operand_b,
            divisible_by,
            true_monkey,
            false_monkey,
        )| {
            MonkeySpec {
                number,
                starting_items,
                operand_a,
                operator,
                operand_b,
                divisible_by,
                true_monkey,
                false_monkey,
            }
        },
    )(string)
}

fn parse_monkeys(string: &str) -> IResult<&str, Vec<MonkeySpec>> {
    separated_list0(tuple((space0, newline)), parse_monkey)(string)
}

fn resolve_operand(operand: Operand, old_worry: u64) -> u64 {
    match operand {
        Operand::Value(n) => n,
        Operand::Old => old_worry,
    }
}

fn run_round(
    monkey_count: usize,
    specs: &HashMap<u64, &MonkeySpec>,
    all_items: &mut HashMap<u64, Vec<u64>>,
    inspected: &mut HashMap<u64, u64>,
) {
    let mut print = true;
    for monkey_index in 0..monkey_count {
        let spec = specs.get(&(monkey_index as u64)).unwrap();
        let items = all_items.get(&(monkey_index as u64)).unwrap().clone();

        let old_inspected = *inspected.get(&(monkey_index as u64)).unwrap();
        inspected.insert(monkey_index as u64, old_inspected + (items.len() as u64));

        for old_worry in items {
            if print { println!("old worry is {}", old_worry) };
            let operand_a = resolve_operand(spec.operand_a, old_worry);
            let operand_b = resolve_operand(spec.operand_b, old_worry);

            let new_worry = match spec.operator {
                Operator::Multiply => operand_a * operand_b,
                Operator::Plus => operand_a + operand_b,
            };

            if print { println!("new worry is {}", new_worry) };
            let less_worried = (new_worry as f64 / 3.0).floor() as u64;

            if print { println!("less worried is {}", less_worried) };
            let test_result = less_worried % spec.divisible_by == 0;


            if print { println!("test is {}", test_result) };
            let target_index = if test_result {
                spec.true_monkey
            } else {
                spec.false_monkey
            };

            if print { println!("throwing to {}", target_index) };

            let mut target_current_items = all_items.get(&target_index).unwrap().clone();
            target_current_items.push(less_worried);

            all_items.insert(target_index, target_current_items);
            print = false;
        }

        all_items.insert(monkey_index as u64, Vec::new());
    }
}

fn main() {
    let string = fs::read_to_string("./input").unwrap();
    let monkey_specs = parse_monkeys(string.as_str()).unwrap().1;
    let monkey_spec_map: HashMap<u64, &MonkeySpec> = HashMap::from_iter(
        monkey_specs
            .iter()
            .map(|monkey| (monkey.number, monkey))
            .into_iter(),
    );

    let mut monkey_inspected_map: HashMap<u64, u64> =
        HashMap::from_iter(monkey_specs.iter().map(|spec| (spec.number, 0)).into_iter());

    let mut monkey_items_map: HashMap<u64, Vec<u64>> = HashMap::from_iter(
        monkey_specs
            .iter()
            .map(|spec| (spec.number, spec.starting_items.clone()))
            .into_iter(),
    );

    let monkey_count = monkey_specs.len();

    for _i in 0..20 {
        run_round(
            monkey_count,
            &monkey_spec_map,
            &mut monkey_items_map,
            &mut monkey_inspected_map,
        );
    }

    let mut throws = Vec::from_iter(monkey_inspected_map.values().into_iter());
    throws.sort_unstable();
    throws.reverse();

    let top = Vec::from_iter(throws.into_iter().take(2));
    let answer: u64 = *top.get(0).unwrap() * *top.get(1).unwrap();

    println!("The answer is {}", answer);
}
