use self::saha::SahaParser;
use peginator::PegParser;
use saha_types::SahaResult;

mod saha;

#[test]
fn test() -> SahaResult {
    let out = SahaParser::parse("input")?;

    Ok(())
}
