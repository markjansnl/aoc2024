use std::{collections::BTreeSet, fs, sync::mpsc::channel, thread, time::Duration};

use aoc2024::*;

use chrono::{Datelike, Local};
use clap::Parser;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Days to run, comma separated. If ommited, only today is run.
    #[arg(short, long, num_args = 0.., value_delimiter = ',')]
    day: Vec<u8>,

    /// Parts to run, comma separated. If omitted, part 1 is run.
    #[arg(short, long, num_args = 0..2, value_delimiter = ',', value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Vec<u8>,

    /// Run all days and all parts
    #[arg(short, long)]
    all: bool,

    /// Download inputs for selected days
    #[arg(long)]
    download: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let days = if cli.all {
        (1..=DAYS).collect()
    } else if cli.day.is_empty() {
        BTreeSet::from([Local::now().day() as u8])
    } else {
        cli.day.iter().copied().collect()
    };

    let parts = if cli.all {
        BTreeSet::from([1, 2])
    } else if cli.part.is_empty() {
        BTreeSet::from([1])
    } else {
        cli.part.iter().copied().collect()
    };

    if cli.download {
        download(&days)?;
    } else {
        let (sender, receiver) = channel();
        days.par_iter().for_each_with(sender, |sender, &day| {
            parts.par_iter().for_each(|&part| {
                sender
                    .send((day, part, run(day, Part::from(part))))
                    .unwrap();
            });
        });

        let mut results = receiver.into_iter().collect::<Vec<_>>();
        results.sort();
        for (day, part, result) in results {
            println!("Day: {day:02}, part {part}: {result}");
        }
    }

    Ok(())
}

fn download(days: &BTreeSet<u8>) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let session = fs::read_to_string(".session")?;

    let mut iter = days.iter().peekable();
    while let Some(day) = iter.next() {
        println!("Downloading day {day}...");

        let mut input = client
            .get(format!("https://adventofcode.com/2024/day/{day}/input"))
            .header("Cookie", format!("session={session}"))
            .send()?
            .text()?;

        input.truncate(input.len() - 1);

        fs::write(format!("src/days/day{day:02}/input.txt"), input)?;

        if iter.peek().is_some() {
            thread::sleep(Duration::from_secs(1));
        }
    }

    Ok(())
}
