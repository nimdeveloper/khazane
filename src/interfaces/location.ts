import { ComplexID, type IComplexID } from "./_base";

export interface ILocation {
    id: IComplexID | null;
    name: string;
}

export class Location implements ILocation {
    constructor(public id: ComplexID, public name: string = "") {}

    static fromInterface(data: ILocation) {
        // (data as any).id = normalizeId((data as any).id);
        return new this(ComplexID.fromInterface(data.id), data.name);
    }
    toInterface(): ILocation {
        return {
            id: this.id.toInterface(),
            name: this.name,
        };
    }
    get label() {
        return this.name;
    }
    get key() {
        return this.id.id;
    }
}
