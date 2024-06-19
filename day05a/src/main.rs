use std::fs;
use std::string::ParseError;
use nom::{AsChar, bytes::complete::{tag, take_while}, error::Error, IResult, Map, multi::separated_list0, Parser};
use std::error::Error as StdError;
use std::str::FromStr;
use nom::character::complete::{digit1, newline, space0};
use nom::combinator::map_res;
use nom::sequence::tuple;

fn parser(s: &str) -> IResult<&str, &str> {
    tag("seeds: ")(s)
}

fn parse_seeds_numbers(s: &str) -> IResult<&str, Vec<&str>, Error<&str>> {
    separated_list0(tag(" "), take_while(AsChar::is_alphanum)).parse(s)
}

fn parse_data(input: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
    fn parse_int(input: &str) -> IResult<&str, i64> {
        map_res(digit1, FromStr::from_str)(input)
    }

    fn parse_triple(input: &str) -> IResult<&str, (i64, i64, i64)> {
        let (input, (a, _, b, _, c, _)) = tuple((parse_int, space0, parse_int, space0, parse_int, space0))(input)?;
        Ok((input, (a, b, c)))
    }

    fn parse_lines(input: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
        separated_list0(newline, parse_triple)(input)
    }

    let (input, lines) = parse_lines(input)?;
    // let (input, _) = tag("\n\n")(input)?;
    Ok((input, lines))
}

fn parse_map_name<'a>(rest: &'a str, map_name: &str) -> IResult<&'a str, &'a str>  {
    let tag_str = format!("\n\n{} map:\n", map_name);
    let x = tag(tag_str.as_str())(rest);
    x
}

fn parse_map<'a>(rest: &'a str, map_name: &str) -> IResult<&'a str, Vec<(i64, i64, i64)>> {
    let (rest, _) = parse_map_name(rest, map_name).unwrap();
    // println!("rest in parse map:{}", rest);
    parse_data(rest)
}

fn calculate_next(vec: &Vec<i64>, map: &Vec<(i64, i64, i64)>) -> Vec<i64> {
    let mut res: Vec<i64> = Vec::new();

    for s in vec {
        let mut min: i64 = i64::MAX;
        let mut found: bool = false;

        for (soil, seed, range) in map {
            if *s >= *seed && *s <= *seed + *range {
                let dif = soil - seed;
                min = min.min(*s + dif);
                found = true;
            } else {
                continue
            }
        }

        if !found {
            res.push(*s);
        } else {
            res.push(min);
        }
    }

    res
}

fn main() -> Result<(), Box<dyn StdError>> {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let input_str: &str = &input;

    let (rest, parsed) = parser(input_str).unwrap();

    let (rest, numbers) = parse_seeds_numbers(rest).unwrap();

    let seeds_numbers: Vec<i64> = numbers.iter().flat_map(|s| s.parse::<i64>().ok()).collect();

    // println!("seeds numbers:{:#?}", seeds_numbers);

    let (rest, seed_to_soil) = parse_map(rest, "seed-to-soil").unwrap();
    let (rest, soil_to_fertilizer) = parse_map(rest, "soil-to-fertilizer").unwrap();
    let (rest, fertilizer_to_water) = parse_map(rest, "fertilizer-to-water").unwrap();
    let (rest, water_to_light) = parse_map(rest, "water-to-light").unwrap();
    let (rest, light_to_temperature) = parse_map(rest, "light-to-temperature").unwrap();
    let (rest, temperature_to_humidity) = parse_map(rest, "temperature-to-humidity").unwrap();
    let (rest, humidity_to_location) = parse_map(rest, "humidity-to-location").unwrap();


    // dbg!(seed_to_soil);
    // dbg!(soil_to_fertilizer);
    // dbg!(fertilizer_to_water);
    // dbg!(water_to_light);
    // dbg!(light_to_temperature);
    // dbg!(temperature_to_humidity);
    // dbg!(humidity_to_location);

    // println!("rest:{}", rest);

    let next: Vec<i64> = calculate_next(&seeds_numbers, &seed_to_soil);
    // println!("next: {:#?}", next);
    let next: Vec<i64> = calculate_next(&next, &soil_to_fertilizer);
    // println!("next: {:#?}", next);
    let next: Vec<i64> = calculate_next(&next, &fertilizer_to_water);
    let next: Vec<i64> = calculate_next(&next, &water_to_light);
    let next: Vec<i64> = calculate_next(&next, &light_to_temperature);
    let next: Vec<i64> = calculate_next(&next, &temperature_to_humidity);
    let next: Vec<i64> = calculate_next(&next, &humidity_to_location);
    println!("{}", next.iter().min().unwrap());


    Ok(())
}