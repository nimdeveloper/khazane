export interface ILocation {
    id: string;
    name: string;
}

export class Location implements ILocation {
    constructor(public id: string, public name: string = "") {}

    static fromInterface(data: ILocation) {
        (data as any).id = normalizeId((data as any).id);
        return new this(data.id, data.name);
    }
    toInterface(): ILocation {
        return {
            id: this.id,
            name: this.name,
        };
    }
    get label() {
        return this.name;
    }
    get value() {
        return this.id;
    }
}
