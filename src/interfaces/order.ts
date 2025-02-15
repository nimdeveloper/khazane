import { Person, type IPerson } from "./person";
import { ProductUnit, type IProductUnit } from "./product";

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
    type: string | null;
    delivery: IPerson | null;
    recipient: IPerson | null;
    citation_number: string | null;
    manager: IPerson | null;
    approvers: IRole[];
    goods: IOrderProduct[];
    document_date: string | null;
    document_number: string | null;
}

export class Order implements IOrder {
    image?: string | File;

    constructor(
        public key: IOrder["key"],
        public type: IOrder["type"] = null,
        public delivery: Person | null = null,
        public recipient: Person | null = null,
        public citation_number: IOrder["citation_number"] = null,
        public manager: Person | null = null,
        public approvers: (IRole & { person: Person })[] = [],
        public goods: (IOrderProduct & { product: ProductUnit })[] = [],
        public document_date: IOrder["document_date"] = null,
        public document_number: IOrder["document_number"] = null
    ) {}
    static fromInterface(data: IOrder) {
        return new this(
            data.key,
            data.type,
            data.delivery ? Person.fromInterface(data.delivery) : null,
            data.recipient ? Person.fromInterface(data.recipient) : null,
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
            data.document_number
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
