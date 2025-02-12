export interface IWareHouse {
    name: string;
    color?: {
        key: string;
        code: string;
    };
    shorthand?: string;
    key: string;
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
        public key: string = "",
        public name: string = "",
        public shorthand: string = "",
        public color: IWareHouse["color"] = undefined
    ) {}

    static fromInterface(data: IWareHouse) {
        return new WareHouse(data.key, data.name, data.shorthand, data.color);
    }
    public toInterface(): IWareHouse {
        return {
            name: this.name,
            color: this.color,
            shorthand: this.shorthand,
            key: this.key,
        };
    }
    get label() {
        return this.shorthand ? this.shorthand : this.name;
    }
    get value() {
        return this.key;
    }
}
