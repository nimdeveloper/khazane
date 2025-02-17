import { MeasurementUnit, type IMeasurementUnit } from "./measurement-unit";
import { WareHouse, type IWareHouse } from "./warehouse";

export interface IProductUnit {
    key: string;
    title: string;
    code: string;
    unit: IMeasurementUnit | null;
    basePrice: number;
    inventory: number;
    initialInventory: number;
    category: IProductCategory | null;
    status: "active" | "draft" | "inactive";
    image?: string | File;
    ware_houses?: IWareHouseQuantity[];
}

export interface IWareHouseQuantity {
    warehouse: IWareHouse | null;
    quantity: number;
}

export class ProductUnit implements IProductUnit {
    image?: string | File;

    constructor(
        public title: string,
        public unit: MeasurementUnit | null = null,
        public key: string = "",
        public code: string = "",
        public basePrice: number = 0,
        public inventory: number = 0,
        public initialInventory: number = 0,
        public category: ProductCategory | null = null,
        public status: IProductUnit["status"] = "draft",
        public ware_houses: Array<
            IWareHouseQuantity & { warehouse: WareHouse | null }
        > = []
    ) {}
    static fromInterface(data: IProductUnit) {
        let obj = new ProductUnit(
            data.title,
            data.unit ? MeasurementUnit.fromInterface(data.unit) : null,
            data.key,
            data.code,
            Number(data.basePrice),
            Number(data.inventory),
            Number(data.initialInventory),
            data.category ? ProductCategory.fromInterface(data.category) : null,
            data.status,
            data.ware_houses
                ? Array.from(
                      data.ware_houses.map((each) => ({
                          warehouse: each.warehouse
                              ? WareHouse.fromInterface(each.warehouse)
                              : null,
                          quantity: Number(each.quantity),
                      }))
                  )
                : []
        );
        if (data.image && !(data.image instanceof File)) {
            obj.image = data.image;
        }
        return obj;
    }
    toInterface(): IProductUnit {
        return {
            key: this.key,
            title: this.title,
            unit: this.unit ? this.unit.toInterface() : null,
            code: this.code,
            basePrice: Number(this.basePrice),
            inventory: Number(this.inventory),
            initialInventory: Number(this.initialInventory),
            category: this.category ? this.category.toInterface() : null,
            ...(this.image && !(this.image instanceof File)
                ? { image: this.image }
                : {}),
            status: this.status,
            ware_houses: this.ware_houses
                ? this.ware_houses.map((each) => ({
                      warehouse: each.warehouse
                          ? each.warehouse.toInterface()
                          : null,
                      quantity: Number(each.quantity),
                  }))
                : [],
        };
    }
    write() {
        return JSON.stringify(this.toInterface());
    }
    static read(data: string) {
        if (!data) return new ProductUnit("");
        return this.fromInterface(JSON.parse(data));
    }
    public valid() {
        return true;
    }
    get label() {
        return this.title;
    }
    get value() {
        return this.key;
    }
}

export interface IProductCategory {
    key: string;
    label: string;
}
export class ProductCategory implements IProductCategory {
    constructor(public key: string = "", public label: string = "") {}
    static fromInterface(data: IProductCategory) {
        return new ProductCategory(data.key, data.label);
    }
    public toInterface() {
        return {
            key: this.key,
            label: this.label,
        };
    }
    write() {
        return JSON.stringify(this.toInterface());
    }
    static read(data: string) {
        if (!data) return new ProductCategory("");
        return this.fromInterface(JSON.parse(data));
    }
    get value() {
        return this.key;
    }
}
