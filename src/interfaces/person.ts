export interface IPerson {}

export class Person implements IPerson {
    static fromInterface(data: IPerson) {
        return new this();
    }
    toInterface(): IPerson {
        return {};
    }
    get label() {
        return "";
    }
    get value() {
        return "";
    }
}
