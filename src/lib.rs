use std::f64::INFINITY;

/// Represents a genomic region, containing blocks
pub struct Region {
    pub chrom: String,
    pub start: u64,
    pub end: u64,
    pub blockstarts: Vec<u64>,
    pub blockends: Vec<u64>,
}

impl Region {
    /// Make blocks in region; returns start coordinates
    pub fn get_blockstarts(start: u64, end: u64, blocklen: u64) -> Vec<u64> {
        let blockstarts: Vec<u64> = (start..end)
            .step_by(
                usize::try_from(blocklen)
                    .ok()
                    .expect("should have been able to convert blocklen to u8"),
            )
            .take_while(|start| start + (&blocklen - 1) <= end)
            .collect();

        blockstarts
    }

    /// Creates new region
    pub fn new(chrom: String, start: u64, end: u64, blocklen: u64) -> Region {
        let blockstarts: Vec<u64> = Self::get_blockstarts(start, end, blocklen);
        let blockends: Vec<u64> = blockstarts
            .clone()
            .into_iter()
            .map(|start| start + (blocklen - 1))
            .collect();

        Region {
            chrom: chrom,
            start: start,
            end: end,
            blockstarts: blockstarts,
            blockends: blockends,
        }
    }

    /// Make Block structs from Region struct
    pub fn to_blocks(self) -> Vec<Block> {
        let mut blocks: Vec<Block> = Vec::new();
        let coord_iter = self.blockstarts.iter().zip(self.blockends.iter());
        for (_, (start, end)) in coord_iter.enumerate() {
            blocks.push(Block::new(self.chrom.clone(), *start, *end));
        }

        blocks
    }
}

/// Represents a genomic block, i.e. contiguous sequence
#[derive(Clone)]
pub struct Block {
    pub chrom: String,
    pub start: u64,
    pub end: u64,
}

impl Block {
    /// Create new block
    pub fn new(chrom: String, start: u64, end: u64) -> Block {
        assert!(end > start, "Block can't have negative length");
        Block { chrom, start, end }
    }
}

/// Get distance between two blocks
pub fn block_distance(blocks: (Block, Block)) -> u64 {
    let mut distance = 0;
    if blocks.0.chrom != blocks.1.chrom {
        distance = f64::to_bits(INFINITY);
    } else {
        assert!(blocks.0.start < blocks.1.end);
        distance = blocks.1.start - blocks.0.end;
    }

    distance
}

/// Filter blocks on minimum distance (LD)
pub fn filter_min_dist(blocks: Vec<Block>, min_dist: u64) -> Vec<Block> {
    let mut filtered: Vec<Block> = Vec::new();

    let mut current_idx: usize = 0;
    let mut compare_idx: usize = 1;

    while compare_idx < blocks.len() {
        if block_distance((blocks[current_idx].clone(), blocks[compare_idx].clone())) >= min_dist {
            // If blocks are far enough, add to Vec and move index along
            filtered.push(blocks[current_idx].clone());
            current_idx = compare_idx;
            compare_idx += 1;
        } else {
            // If comparison blocks are too close, compare current block with one block further down
            compare_idx += 1;
        }
    }

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_new_region() {
        let chrom: String = "chr2".to_string();
        let start: u64 = 100;
        let end: u64 = 499;
        let blocklen: u64 = 100;
        let region: Region = Region::new(chrom, start, end, blocklen);
        let expected_blockstarts: Vec<u64> = vec![100, 200, 300, 400];

        for (idx, blockstart) in region.blockstarts.into_iter().enumerate() {
            assert!(blockstart == expected_blockstarts[idx]);
        }
    }

    #[test]
    fn test_block_distance() {
        let block1 = Block::new("1".to_string(), 100, 500);
        let block2 = Block::new("1".to_string(), 600, 1000);

        let expected_dist = 100;
        let calculated_dist = block_distance((block1, block2));

        assert!(expected_dist == calculated_dist);
    }

    #[test]
    fn test_block_distance_diff_chroms() {
        let block1 = Block::new("1".to_string(), 100, 500);
        let block2 = Block::new("2".to_string(), 600, 1000);

        let expected_dist = f64::to_bits(INFINITY);
        let calculated_dist = block_distance((block1, block2));

        assert!(expected_dist == calculated_dist);
    }

    #[test]
    fn test_min_distance() {
        let chrom: String = "chr2".to_string();
        let start: u64 = 100;
        let end: u64 = 5000;
        let blocklen: u64 = 100;
        let min_dist = 1000;
        let region: Region = Region::new(chrom, start, end, blocklen);
        let blocks: Vec<Block> = region.to_blocks();
        let filtered_blocks: Vec<Block> = filter_min_dist(blocks, min_dist);
        let expected_blockstarts: Vec<u64> = vec![100, 1200, 2300, 3400, 4500];

        for (idx, block) in filtered_blocks.into_iter().enumerate() {
            println!("{}, {}", block.start, expected_blockstarts[idx]);
            assert!(block.start == expected_blockstarts[idx]);
        }
    }
}
