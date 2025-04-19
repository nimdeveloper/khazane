export interface IPerson {
    id: Number;
    full_name: string;
}

export class Person implements IPerson {
    constructor(public id: Number, public full_name: string = "") {}

    static fromInterface(data: IPerson) {
        // (data as any).id = normalizeId((data as any).id);
        return new this(data.id, data.full_name);
    }
    toInterface(): IPerson {
        return {
            id: this.id,
            full_name: this.full_name,
        };
    }
    get label() {
        return this.full_name;
    }
    get key() {
        return this.id;
    }
    get name() {
        return this.full_name;
    }
    set name(value) {
        this.full_name = value;
    }
}
