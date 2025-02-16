import { Person, type IPerson } from "./person";
import { ProductUnit, type IProductUnit } from "./product";
import { instanceOfIWareHouse, WareHouse, type IWareHouse } from "./warehouse";

export interface IRole {
    name: string;
    person: IPerson;
}
export interface IOrderProduct {
    product: IProductUnit;
    quantity: number;
}
export interface IOrder {
    key: string;
    type?: string;
    delivery: IPerson | IWareHouse | null;
    recipient: IPerson | IWareHouse | null;
    citation_number?: string;
    manager: IPerson | null;
    approvers: IRole[];
    goods: IOrderProduct[];
    document_date?: string;
    document_number?: string;
    status: string;
}

export class Order implements IOrder {
    image?: string | File;

    constructor(
        public key: IOrder["key"],
        public type?: IOrder["type"],
        public delivery: Person | WareHouse | null = null,
        public recipient: Person | WareHouse | null = null,
        public citation_number?: IOrder["citation_number"],
        public manager: Person | null = null,
        public approvers: (IRole & { person: Person })[] = [],
        public goods: (IOrderProduct & { product: ProductUnit })[] = [],
        public document_date?: IOrder["document_date"],
        public document_number?: IOrder["document_number"],
        public status = "draft"
    ) {}
    static autoPersonWareHouse(data: IPerson | IWareHouse): Person | WareHouse {
        if (instanceOfIWareHouse(data)) {
            return WareHouse.fromInterface(data as IWareHouse);
        }
        return Person.fromInterface(data as IPerson);
    }
    static fromInterface(data: IOrder) {
        return new this(
            data.key,
            data.type,
            data.delivery ? this.autoPersonWareHouse(data.delivery) : null,
            data.recipient ? this.autoPersonWareHouse(data.recipient) : null,
            data.citation_number,
            data.manager ? Person.fromInterface(data.manager) : null,
            data.approvers
                ? data.approvers.map((each) => ({
                      name: each.name,
                      person: Person.fromInterface(each.person),
                  }))
                : [],
            data.goods
                ? data.goods.map((each) => ({
                      quantity: each.quantity,
                      product: ProductUnit.fromInterface(each.product),
                  }))
                : [],
            data.document_date,
            data.document_number,
            data.status
        );
    }
    toInterface(): IOrder {
        return {
            key: this.key,
            type: this.type,
            delivery: this.delivery ? this.delivery.toInterface() : null,
            recipient: this.recipient ? this.recipient.toInterface() : null,
            citation_number: this.citation_number,
            manager: this.manager ? this.manager.toInterface() : null,
            approvers: this.approvers
                ? this.approvers.map((each) => ({
                      person: each.person.toInterface(),
                      name: each.name,
                  }))
                : [],
            goods: this.goods
                ? this.goods.map((each) => ({
                      product: each.product.toInterface(),
                      quantity: each.quantity,
                  }))
                : [],
            document_date: this.document_date,
            document_number: this.document_number,
            status: this.status,
        };
    }
    write() {
        return JSON.stringify(this.toInterface());
    }
    static read(data: string) {
        if (!data) return new Order("");
        return this.fromInterface(JSON.parse(data));
    }
    public valid() {
        return true;
    }
}
