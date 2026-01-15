import { Dependency } from './dependency';

// Multiple interfaces that nobody uses
export interface IUselessService {
    doNothing(): void;
}

export interface IAnotherInterface {
    method(): void;
}

export interface IYetAnotherInterface {
    anotherMethod(): void;
}

export interface IFourthInterface {
    fourthMethod(): void;
}

// Only one concrete class implementing all these interfaces
// 0 clients use this, but it depends on external concrete class
export class UselessService implements IUselessService, IAnotherInterface, IYetAnotherInterface, IFourthInterface {
    constructor(private dep: Dependency) {}
    doNothing() {}
    method() {}
    anotherMethod() {}
    fourthMethod() {}
}
