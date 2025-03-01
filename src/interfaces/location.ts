export interface ILocation {
    key: string;
    name: string;
}

export class Location implements ILocation {
    constructor(public key: string, public name: string = "") {}

    static fromInterface(data: ILocation) {
        return new this(data.key, data.name);
    }
    toInterface(): ILocation {
        return {
            key: this.key,
            name: this.name,
        };
    }
    get label() {
        return this.name;
    }
    get value() {
        return this.key;
    }
}
