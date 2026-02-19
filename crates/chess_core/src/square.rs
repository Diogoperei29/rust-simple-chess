#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    rank: u8, // 0-7 (1-8)
    file: u8, // 0-7 (a-h)
}

impl Square {
    pub fn new(rank: u8, file: u8) -> Result<Self, &'static str> {
        if rank < 8 && file < 8 {
            Ok (Self { rank, file })
        } else {
            Err ("Tried to create a square out of bounds\n")
        }
    }

    pub fn new_from_notation(notation: &str) -> Result<Self, &'static str> {
        let bytes = notation.as_bytes();
        if bytes.len() != 2 {
            return Err("Error trying to read position notation\n");
        }

        let file = ((bytes[0] as char).to_ascii_lowercase() as u8) - b'a';
        let rank = (bytes[1] as u8) - b'1';

        Self::new(rank, file)
    }

    pub fn to_notation(&self) -> String {
        format!("{}{}", (b'a' + self.file) as char, self.rank + 1)
    }

    pub fn offset(&self, rank_offset: i8, file_offset: i8) -> Result<Self, &'static str> {
        let new_rank = self.rank as i8 + rank_offset;
        let new_file = self.file as i8 + file_offset;

        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
            Ok (Self { rank: new_rank as u8, file: new_file as u8 })
        } else {
            Err ("Tried to create a square out of bounds\n")
        }
    }

    pub fn rank(&self) -> u8 { self.rank }
    pub fn file(&self) -> u8 { self.file }
    pub fn rank_file(&self) -> (u8, u8) { return (self.rank, self.file) }

}