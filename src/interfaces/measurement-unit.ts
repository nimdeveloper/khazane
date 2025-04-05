import { ComplexID, type IComplexID } from "./_base";

export interface IMeasurementUnit {
    id: IComplexID | null;
    title: string;
}
export class MeasurementUnit implements IMeasurementUnit {
    constructor(public id: ComplexID, public title: string) {}
    static fromInterface(data: IMeasurementUnit) {
        // (data as any).id = normalizeId((data as any).id);
        return new MeasurementUnit(
            ComplexID.fromInterface(data.id),
            data.title
        );
    }
    toInterface(): IMeasurementUnit {
        return {
            id: this.id.toInterface(),
            title: this.title,
        };
    }
    get label() {
        return this.title;
    }
    get key() {
        return this.id.id;
    }
}
