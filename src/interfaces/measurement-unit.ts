export interface IMeasurementUnit {
    key: string;
    title: string;
}
export class MeasurementUnit {
    constructor(public key: string, public title: string) {}
    static fromInterface(data: IMeasurementUnit) {
        return new MeasurementUnit(data.key, data.title);
    }
    toInterface(): IMeasurementUnit {
        return {
            key: this.key,
            title: this.title,
        };
    }
    get label() {
        return this.title;
    }
    get value() {
        return this.key;
    }
}
