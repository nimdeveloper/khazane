import { useStorage } from "@vueuse/core";
import { ComplexID } from "~/interfaces/_base";
import { Order } from "~/interfaces/order";

export const useTempOrder = () => {
    const storage = useStorage(
        "new-order-temp-item",
        new Order(ComplexID.empty()),
        sessionStorage,
        {
            serializer: {
                read: (v: any) => Order.read(v),
                write: (v: any) => (v ? v.write() : undefined),
            },
        }
    );
    return { storage };
};
