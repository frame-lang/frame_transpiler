export interface SystemSnapshot {
    schemaVersion: number;
    systemName: string;
    state: string;
    stateArgs: any;
    domainState: Record<string, any>;
    stack: Array<{
        state: string;
        stateArgs: any;
    }>;
}
export interface SnapshotSystemOptions {
    systemName?: string;
    domainKeys?: Iterable<string>;
    encodeDomain?: (system: any) => Record<string, any>;
}
export interface RestoreSystemOptions {
    domainKeys?: Iterable<string>;
    decodeDomain?: (snapshot: SystemSnapshot, system: any) => void;
}
export declare function snapshotSystem(system: any, opts?: SnapshotSystemOptions): SystemSnapshot;
export declare function restoreSystem(snapshot: SystemSnapshot, systemFactory: () => any, opts?: RestoreSystemOptions): any;
export declare function snapshotToJson(snapshot: SystemSnapshot, indent?: number): string;
export declare function snapshotFromJson(text: string): SystemSnapshot;
export interface SnapshotComparison {
    equal: boolean;
    differences: string[];
}
export declare function compareSnapshots(a: SystemSnapshot, b: SystemSnapshot): SnapshotComparison;
