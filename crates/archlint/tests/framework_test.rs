use archlint::detectors::lcom::LcomDetector;
use archlint::detectors::Detector;
use archlint::detectors::{abstractness::AbstractnessViolationDetector, CodeRange};
use archlint::engine::AnalysisContext;
use archlint::framework::preset_loader::PresetLoader;
use archlint::parser::{ClassSymbol, FileSymbols, MethodSymbol};
use std::path::PathBuf;

#[test]
fn test_lcom_skips_nestjs_controller() {
    let mut ctx = AnalysisContext::default_for_test();
    let controller_path = PathBuf::from("src/app.controller.ts");

    // 1. Setup framework context with preset
    let preset = PresetLoader::load_builtin("nestjs").unwrap();
    ctx.presets = vec![preset.clone()];

    // Merge preset rules into config for the resolver to work
    for (name, rule) in preset.rules {
        ctx.config.rules.insert(name, rule);
    }
    for ov in preset.overrides {
        ctx.config.overrides.push(ov);
    }

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

    // Should be 0 because it matches the override in NestJS preset for controllers
    assert_eq!(smells.len(), 0);
}

#[test]
fn test_abstractness_skips_nestjs_dto() {
    let mut ctx = AnalysisContext::default_for_test();
    let dto_path = PathBuf::from("src/user.dto.ts");

    // 1. Setup framework context
    let preset = PresetLoader::load_builtin("nestjs").unwrap();
    ctx.presets = vec![preset.clone()];

    // Merge preset rules and overrides
    for (name, rule) in preset.rules {
        ctx.config.rules.insert(name, rule);
    }
    for ov in preset.overrides {
        ctx.config.overrides.push(ov);
    }

    // 2. Add symbols that would trigger abstractness violation
    let symbols = FileSymbols::default();
    ctx.file_symbols_mut().insert(dto_path.clone(), symbols);
    ctx.graph_mut().add_file(&dto_path);

    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    // Should be 0 because it matches the override in NestJS preset for DTOs
    assert_eq!(smells.len(), 0);
}
