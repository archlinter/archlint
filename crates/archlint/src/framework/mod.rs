pub mod classifier;
pub mod detector;
pub mod presets;
pub mod selector;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Framework {
    NestJS,
    NextJS,
    Express,
    React,
    Angular,
    Vue,
    TypeORM,
    Prisma,
    Oclif,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileType {
    Controller,
    Service,
    Module,
    Entity,
    Repository,
    DTO,
    Interface,
    Types,
    Config,
    Middleware,
    Guard,
    Pipe,
    Interceptor,
    Decorator,
    Component,
    Hook,
    Page,
    ApiRoute,
    Migration,
    Test,
    Event,
    Exception,
    CliCommand,
    CliHook,
    Unknown,
}
