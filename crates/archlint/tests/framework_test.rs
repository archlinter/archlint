use archlint::detectors::lcom::LcomDetector;
use archlint::detectors::Detector;
use archlint::detectors::{abstractness::AbstractnessViolationDetector, CodeRange};
use archlint::engine::AnalysisContext;
use archlint::framework::{FileType, Framework};
use archlint::parser::{ClassSymbol, FileSymbols, MethodSymbol};
use std::path::PathBuf;

#[test]
fn test_lcom_skips_nestjs_controller() {
    let mut ctx = AnalysisContext::default_for_test();
    let controller_path = PathBuf::from("src/app.controller.ts");

    // 1. Setup framework context
    ctx.detected_frameworks = vec![Framework::NestJS];
    ctx.file_types
        .insert(controller_path.clone(), FileType::Controller);

    // 2. Add a class with low cohesion (3 unconnected methods)
    let mut class = ClassSymbol::new("AppController");
    class.methods.push(MethodSymbol::new(
        "method1",
        0,
        0,
        CodeRange::default(),
        false,
        false,
        None,
        false,
    ));
    class.methods.push(MethodSymbol::new(
        "method2",
        0,
        0,
        CodeRange::default(),
        false,
        false,
        None,
        false,
    ));
    class.methods.push(MethodSymbol::new(
        "method3",
        0,
        0,
        CodeRange::default(),
        false,
        false,
        None,
        false,
    ));

    let mut symbols = FileSymbols::default();
    symbols.classes.push(class);
    ctx.file_symbols_mut()
        .insert(controller_path.clone(), symbols);

    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    // Should be 0 because it's a NestJS Controller
    assert_eq!(smells.len(), 0);
}

#[test]
fn test_abstractness_skips_nestjs_dto() {
    let mut ctx = AnalysisContext::default_for_test();
    let dto_path = PathBuf::from("src/user.dto.ts");

    // 1. Setup framework context
    ctx.detected_frameworks = vec![Framework::NestJS];
    ctx.file_types.insert(dto_path.clone(), FileType::DTO);

    // 2. Add symbols that would trigger abstractness violation
    // (DTO is usually 100% abstract but has no dependants, which puts it in Zone of Pain if it was a core module)
    let symbols = FileSymbols::default();
    ctx.file_symbols_mut().insert(dto_path.clone(), symbols);
    ctx.graph_mut().add_file(&dto_path);

    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    // Should be 0 because it's a NestJS DTO
    assert_eq!(smells.len(), 0);
}
