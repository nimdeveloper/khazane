export interface IMeasurementUnit {
    id: string;
    title: string;
}
export class MeasurementUnit implements IMeasurementUnit {
    constructor(public id: string, public title: string) {}
    static fromInterface(data: IMeasurementUnit) {
        (data as any).id = normalizeId((data as any).id);
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
    get value() {
        return this.id;
    }
}
