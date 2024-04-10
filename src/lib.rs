use bio::io::bed;
use std::{env, process};

/// Represents a genomic region, containing blocks
pub struct Region<'a> {
    pub chrom: &'a str,
    pub start: u64,
    pub end: u64,
    pub blockstarts: Vec<u64>,
    pub blockends: Vec<u64>
}

impl <'a> Region<'a> {

    /// Make blocks in region; returns start coordinates
    pub fn get_blockstarts(start: u64, end: u64, blocklen: u64) -> Vec<u64> {
        let blockstarts: Vec<u64> = (start..end)
            .step_by(usize::try_from(blocklen).ok()
            .expect("should have been able to convert blocklen to u8"))
            .take_while(|start| start+&blocklen <= end)
            .collect();

        blockstarts
    }

    /// Creates new region
    pub fn new(chrom: &'a str, start: &'a u64, end: &'a u64, blocklen: &'a u64) -> Region<'a> {
        let blockstarts:Vec<u64> = Self::get_blockstarts(*start, *end, *blocklen);
        let blockends: Vec<u64> = blockstarts.clone().into_iter()
            .map(|start| start+blocklen)
            .collect();

        Region{
            chrom: chrom,
            start: *start,
            end: *end,
            blockstarts: blockstarts,
            blockends: blockends
        }
    }
}


/// Represents all blocks in region
// struct Intervals {
//     chroms: Vec<String>, 
//     starts: Vec<u64>,
//     ends: Vec<u64>,
// }

// impl Intervals {

//     /// Create struct of block intervals
//     fn new(&self, regions: Vec<Region>) -> Intervals {
//         Intervals{
//             starts: regions.into_iter().map(|region| region.blockstarts).flatten().collect(),
//             ends: regions.into_iter().map(|region| region.blockends).flatten().collect(),
//             chroms: regions.into_iter().map(|region| region.chrom).flatten().collect(),
//         }

//     }

//     /// Filter on minimum distance between blocks
//     fn filter_min_dist(&self) {
//         // this will require some more thinking
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_new_region() {
        let chrom: &str = "chr2";
        let start: u64 = 100;
        let end: u64 = 499;
        let blocklen: u64 = 100;
        let region: Region = Region::new(chrom, &start, &end, &blocklen);
        let expected_blockstarts: Vec<u64> = vec![100, 200, 300, 400];

        for (idx, blockstart) in region.blockstarts.into_iter().enumerate() {
            assert!(blockstart == expected_blockstarts[idx]);
        }
    }
}
