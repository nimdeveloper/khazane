export interface IPerson {
    key: string;
    full_name: string;
}

export class Person implements IPerson {
    constructor(public key: string, public full_name: string = "") {}

    static fromInterface(data: IPerson) {
        return new this(data.key, data.full_name);
    }
    toInterface(): IPerson {
        return {
            key: this.key,
            full_name: this.full_name,
        };
    }
    get label() {
        return this.full_name;
    }
    get value() {
        return this.key;
    }
    get name() {
        return this.full_name;
    }
    set name(value) {
        this.full_name = value;
    }
}
