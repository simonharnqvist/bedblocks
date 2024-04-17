use bedblocks::{filter_min_dist, Block, Region};
use bio::io::bed;
use clap::Parser;
use clap_stdin::MaybeStdin;
use std;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of input BED file
    #[arg(short, long)]
    input: MaybeStdin<PathBuf>,

    /// Length of blocks to make
    #[arg(short, long)]
    blocklength: u64,

    /// Minimum distance between blocks
    #[arg(short, long, default_value_t = 0)]
    min_dist: u64,
}

fn main() {
    let args = Args::parse();
    let input = args.input.to_str().expect("Could not parse file or stdin");
    let mut reader = bed::Reader::from_file(input).expect("Unable to open BED file");

    let records = reader.records().map(|record| record.unwrap());
    for record in records {
        let region = Region::new(
            record.chrom().to_string(),
            record.start(),
            record.end(),
            args.blocklength,
        );

        let blocks: Vec<Block> = Region::to_blocks(region);
        let filtered_blocks = filter_min_dist(blocks, args.min_dist);

        let stdout = io::stdout();
        let mut handle = stdout.lock();
        for block in filtered_blocks {
            writeln!(handle, "{}\t{}\t{}", &block.chrom, &block.start, &block.end)
                .expect("Unable to write to stdout");
        }
        drop(handle);
    }
}
