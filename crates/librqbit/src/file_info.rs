use std::path::PathBuf;

use librqbit_core::torrent_metainfo::FileDetailsAttrs;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub relative_filename: PathBuf,
    pub offset_in_torrent: u64,
    pub piece_range: std::ops::Range<u32>,
    pub attrs: FileDetailsAttrs,
    pub len: u64,
}

// Iterate file pieces in a streaming-friendly order: first, last, then the rest.
fn iter_piece_priorities(range: std::ops::Range<usize>) -> impl Iterator<Item = usize> {
    let r = range;
    use std::iter::once;

    let len = r.len();
    let first = once(r.start);
    let last = once(r.end.saturating_sub(1));
    let mid = r.clone().skip(1).take(len.saturating_sub(2));

    first.chain(last).chain(mid).take(len)
}

impl FileInfo {
    pub fn piece_range_usize(&self) -> std::ops::Range<usize> {
        self.piece_range.start as usize..self.piece_range.end as usize
    }

    pub fn iter_piece_priorities(&self) -> impl Iterator<Item = usize> {
        iter_piece_priorities(self.piece_range_usize())
    }
}

#[cfg(test)]
mod tests {
    use super::iter_piece_priorities;

    #[test]
    fn test_iter_piece_priorities() {
        let it = |r: std::ops::Range<usize>| -> Vec<usize> { iter_piece_priorities(r).collect() };
        assert_eq!(it(0..0), Vec::<usize>::new());

        assert_eq!(it(0..1), vec![0]);
        assert_eq!(it(0..2), vec![0, 1]);
        assert_eq!(it(0..3), vec![0, 2, 1]);
        assert_eq!(it(0..4), vec![0, 3, 1, 2]);
    }
}
