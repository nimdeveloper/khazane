import { MeasurementUnit, type IMeasurementUnit } from "./measurement-unit";
import { WareHouse, type IWareHouse } from "./warehouse";

export interface IProductUnit {
    id: Number;
    title: string;
    code: string;
    unit: IMeasurementUnit | null;
    base_price: number;
    inventory: number;
    initial_inventory: number;
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
        public id: Number = 0,
        public code: string = "",
        public base_price: number = 0,
        public inventory: number = 0,
        public initial_inventory: number = 0,
        public category: ProductCategory | null = null,
        public status: IProductUnit["status"] = "draft",
        public ware_houses: Array<
            IWareHouseQuantity & { warehouse: WareHouse | null }
        > = []
    ) {}
    static fromInterface(data: IProductUnit) {
        // (data as any).id = normalizeId((data as any).id);
        let obj = new ProductUnit(
            data.title,
            data.unit ? MeasurementUnit.fromInterface(data.unit) : null,
            data.id,
            data.code,
            Number(data.base_price),
            Number(data.inventory),
            Number(data.initial_inventory),
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
            id: this.id,
            title: this.title,
            unit: this.unit ? this.unit.toInterface() : null,
            code: this.code,
            base_price: Number(this.base_price),
            inventory: Number(this.inventory),
            initial_inventory: Number(this.initial_inventory),
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
    get key() {
        return this.id;
    }
}

export interface IProductCategory {
    id: Number;
    label: string;
}
export class ProductCategory implements IProductCategory {
    constructor(public id: Number = 0, public label: string = "") {}
    static fromInterface(data: IProductCategory) {
        // (data as any).id = normalizeId((data as any).id);
        return new ProductCategory(data.id, data.label);
    }
    public toInterface() {
        return {
            id: this.id,
            label: this.label,
        };
    }
    write() {
        return JSON.stringify(this.toInterface());
    }
    static read(data: string) {
        if (!data) return new ProductCategory(0);
        return this.fromInterface(JSON.parse(data));
    }
    get key() {
        return this.id;
    }
}
