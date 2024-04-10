use bedblocks::Region;
use std::{env, process};
use bio::io::bed;

pub struct Config {
    pub path: String,
    pub blocklength: u64,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        assert!(args.len() == 3, "Expected 2 arguments");

        let path = args[1].clone();
        let blocklength: u64 = args[2].clone().parse().unwrap();

        Ok(Config{ path, blocklength})
    }
}



fn main() {

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });

    let mut reader = bed::Reader::from_file(config.path).expect("Should have been able to read");

    let records = reader.records().map(|record| record.unwrap());

    let regions: Vec<Region> = records.map(|record| Region::new(&record.chrom(), &record.start(), &record.end(), &config.blocklength))
        .into_iter()
        .collect();

    regions.into_iter().map(|region| println!("{:#?}", region.blockstarts));
}


//      for record in reader.records() {
//          let rec = record.expect("Error reading record");
//          println!("{}", rec.chrom());
//          let region = Region{
//              chrom: rec.chrom(),
//              start: rec.start(),
//              end: rec.end()};

//          let starts = region.get_starts(config.blocklength);

//          for start in starts {
//             println!("{}\t{}", region.chrom, start)
//         }
//     }
// }
