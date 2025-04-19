export interface IWareHouse {
    name: string;
    color?: {
        key: string;
        code: string;
    };
    shorthand?: string;
    id: Number;
}
export function instanceOfIWareHouse(object: any): Boolean {
    return object && "shorthand" in object;
}
export class WareHouse implements IWareHouse {
    static COLOR_PALATES = [
        {
            key: "lemon",
            code: "#F9DB4F",
        },
        {
            key: "lightBlue",
            code: "#3FFDFF",
        },
        {
            key: "purple",
            code: "#F98CFF",
        },
        {
            key: "green",
            code: "#56F3AF",
        },
        {
            key: "orange",
            code: "#E0955F",
        },
    ];
    constructor(
        public id: Number = 0,
        public name: string = "",
        public shorthand: string = "",
        public color: IWareHouse["color"] = undefined
    ) {}

    static fromInterface(data: IWareHouse) {
        // (data as any).id = normalizeId((data as any).id);
        return new WareHouse(data.id, data.name, data.shorthand, data.color);
    }
    public toInterface(): IWareHouse {
        return {
            name: this.name,
            color: this.color,
            shorthand: this.shorthand,
            id: this.id,
        };
    }
    get label() {
        return this.shorthand ? this.shorthand : this.name;
    }
    get key() {
        return this.id;
    }
}
