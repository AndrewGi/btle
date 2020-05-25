#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub struct Index(u8);
impl Index {
    pub const INDEX_MAX: u8 = 39;
    pub const ZERO: Index = Index(0);
    pub const MAX: Index = Index(Self::INDEX_MAX);
    pub const ADVERTISING: [Index; 3] = [Index(37), Index(38), Index(39)];
    pub fn new(index: u8) -> Index {
        assert!(index < Self::INDEX_MAX, "index overflow `{:?}`", index);
        Index(index)
    }
    pub fn new_checked(index: u8) -> Option<Index> {
        if index > Self::INDEX_MAX {
            None
        } else {
            Some(Index(index))
        }
    }
    pub fn new_clamped(index: u8) -> Index {
        Self::new_checked(index).unwrap_or(Self::MAX)
    }
    /// Returns channel frequency in MHz.
    /// # Example
    /// ```
    /// use btle::channel::Index;
    /// assert_eq!(Index::new(37).frequency(), 2402);
    /// assert_eq!(Index::new(0).frequency(), 2404);
    /// assert_eq!(Index::new(10).frequency(), 2424);
    /// assert_eq!(Index::new(38).frequency(), 2426);
    /// assert_eq!(Index::new(11).frequency(), 2428);
    /// assert_eq!(Index::new(36).frequency(), 2478);
    /// ```
    pub fn frequency(self) -> usize {
        match self.0 {
            37 => 2402,
            i if i < 11 => 2404 + 2 * usize::from(i),
            38 => 2426,
            39 => 2800,
            i => 2404 + 2 * (usize::from(i) + 1),
        }
    }
    pub fn is_advertising(self) -> bool {
        Self::ADVERTISING.contains(&self)
    }
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}
impl From<Index> for u8 {
    fn from(i: Index) -> Self {
        i.0
    }
}
