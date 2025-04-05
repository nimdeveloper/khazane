export interface IComplexID {
    tb: string;
    id: String;
}

export class ComplexID implements IComplexID {
    _empty = false;
    constructor(public tb: string, public id: string) {}

    static fromInterface(data: IComplexID | null) {
        if (data && data.id) {
            if ((data.id as any).String) {
                return new this(data.tb, (data.id as any).String);
            }
        }
        throw Error("Unsupported key type!");
    }
    static empty() {
        let obj = new this("", "");
        obj._empty = true;
        return obj;
    }
    toInterface(): IComplexID | null {
        if (this._empty) return null;
        return {
            id: this.id,
            tb: this.tb,
        };
    }
}
