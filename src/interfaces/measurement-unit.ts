export interface IMeasurementUnit {
    id: Number;
    title: string;
}
export class MeasurementUnit implements IMeasurementUnit {
    constructor(public id: Number, public title: string) {}
    static fromInterface(data: IMeasurementUnit) {
        // (data as any).id = normalizeId((data as any).id);
        return new MeasurementUnit(data.id, data.title);
    }
    toInterface(): IMeasurementUnit {
        return {
            id: this.id,
            title: this.title,
        };
    }
    get label() {
        return this.title;
    }
    get key() {
        return this.id;
    }
}
