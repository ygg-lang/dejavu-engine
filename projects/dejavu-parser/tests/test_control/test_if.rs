struct HelloTemplate {}

impl core::fmt::Display for HelloTemplate {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("TextWordNode { span: 0..1 }")?;
        f.write_str("TextSpaceNode { span: 1..2 }")?;
        f.write_str("TextWordNode { span: 2..8 }")?;
        f.write_str("TextSpaceNode { span: 8..9 }")?;
        f.write_str("TextWordNode { span: 9..11 }")?;
        f.write_str("TextSpaceNode { span: 11..12 }")?;
        f.write_str("TextWordNode { span: 12..20 }")?;
        f.write_str("TextSpaceNode { span: 20..21 }")?;
        f.write_str("TextSpaceNode { span: 55..57 }")?;
        f.write_str("TextWordNode { span: 57..58 }")?;
        f.write_str("TextSpaceNode { span: 58..59 }")?;
        f.write_str("TextWordNode { span: 59..61 }")?;
        f.write_str("TextSpaceNode { span: 61..62 }")?;
        f.write_str("TextWordNode { span: 62..66 }")?;
        f.write_str("TextSpaceNode { span: 66..67 }")?;
        f.write_str("TextWordNode { span: 67..74 }")?;
        f.write_str("TextSpaceNode { span: 74..75 }")?;
        f.write_str("TextWordNode { span: 75..76 }")?;
        f.write_str("TextSpaceNode { span: 76..77 }")?;
        f.write_str("TextWordNode { span: 77..83 }")?;
        f.write_str("TextSpaceNode { span: 83..84 }")?;
        f.write_str("TextWordNode { span: 84..86 }")?;
        f.write_str("TextSpaceNode { span: 86..87 }")?;
        f.write_str("TextWordNode { span: 87..95 }")?;
        f.write_str("TextSpaceNode { span: 95..96 }")?;
        f.write_str("TextSpaceNode { span: 154..157 }")?;
        f.write_str("TextWordNode { span: 157..158 }")?;
        f.write_str("TextSpaceNode { span: 158..159 }")?;
        f.write_str("TextWordNode { span: 159..169 }")?;
        f.write_str("TextSpaceNode { span: 169..170 }")?;
        f.write_str("TextWordNode { span: 170..177 }")?;
        f.write_str("TextSpaceNode { span: 177..178 }")?;
        f.write_str("TextWordNode { span: 178..179 }")?;
        f.write_str("TextSpaceNode { span: 179..180 }")?;
        f.write_str("TextWordNode { span: 180..186 }")?;
        f.write_str("TextSpaceNode { span: 186..187 }")?;
        f.write_str("TextWordNode { span: 187..189 }")?;
        f.write_str("TextSpaceNode { span: 189..190 }")?;
        f.write_str("TextWordNode { span: 190..198 }")?;
        f.write_str("TextSpaceNode { span: 198..199 }")?;
        Ok(())
    }
}
