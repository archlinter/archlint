use archlint::parser::tokenizer::{tokenize_and_normalize, CloneTokenizationMode};
use oxc_span::SourceType;
use std::fs;

fn main() {
    let source = fs::read_to_string("/Users/ivan/projects/superprotocol/sp-providers/apps/centralized-provider/src/offers/offers.service.ts").unwrap();
    let source_type = SourceType::from_path("offers.service.ts").unwrap();
    let tokens = tokenize_and_normalize(&source, source_type, CloneTokenizationMode::Exact);
    
    for (i, t) in tokens.iter().enumerate() {
        if t.line >= 209 && t.line <= 228 {
            println!("{:4}: line={:3} col={:2} normalized={:15} seq={:4}", i, t.line, t.column, t.normalized, t.seq);
        }
    }
}
