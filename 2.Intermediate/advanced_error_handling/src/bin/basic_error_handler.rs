#![allow(dead_code)]

use std::{collections::HashMap, io, num::ParseIntError};
// 1. remove unwrap from parse_card_numbers and return result
fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParseIntError> {
    let numbers = card
        .split(" ")
        .map(|s| {
            s.parse()
        })
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(numbers)
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn parse_card(card: &str) -> Result<Card, String> {
    // 2. convert  ParseInt to String via map_err combinator
    let mut numbers = parse_card_numbers(card).map_err(|e| e.to_string())?;

    let len = numbers.len();
    let expected_len = 4;

    // 3. check the length of the split array
    if len != expected_len {
        return Err(format!(
            "Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}"
        ));
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv
    })
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, String> {
    // 4. Convert Option to Result type via ok_or() combinator
    let card_string = credit_cards.get(name)
        .ok_or(format!("No credit card was found for {name}."))?;

    let card = parse_card(card_string)?;

    Ok(card)
}

fn main() {
    let credit_cards = HashMap::from([
        ("Amy", "1234567 12 16 123"),
        ("Tim", "1234567 0616 123"),
        ("Bob", "1234567 Dec 08 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    // 5. Use match statement to show errorA
    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => println!("{err}"),
    }
}