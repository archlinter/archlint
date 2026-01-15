export interface IUnused {
    /** marker to avoid empty interface lint; optional to keep class unchanged */
    _unused?: never;
}
export class Unused implements IUnused {}
