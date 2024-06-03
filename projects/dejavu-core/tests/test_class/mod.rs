use super::*;
use dejavu_parser::dejavu::{NexusParser, NexusRule};

#[test]
fn test_unicode() {
    let cst = NexusParser::parse_cst(include_str!("test_class.djv"), NexusRule::Root).unwrap();
    println!("Short Form:\n{}", cst);
    let first = RootNode::from_cst(cst).unwrap();
    println!("{:#?}", first)
}

#[test]
fn test_unicode2() {
    let cst = NexusParser::parse_cst("{% export parse_template %}", NexusRule::TemplateExport).unwrap();
    println!("{}", cst);
    let cst = NexusParser::parse_cst("{% export parse_template to rust %}", NexusRule::TemplateExport).unwrap();
    println!("{}", cst);
    let cst = NexusParser::parse_cst("{% export parse_template by class Template %}", NexusRule::TemplateExport).unwrap();
    println!("{}", cst);
    let cst =
        NexusParser::parse_cst("{% export parse_template by class Template by trait %}", NexusRule::TemplateExport).unwrap();
    println!("{}", cst);
    let cst =
        NexusParser::parse_cst("{% export parse_template by class Template by trait TemplateExt %}", NexusRule::TemplateExport)
            .unwrap();
    println!("{}", cst);
}
