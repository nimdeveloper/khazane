import { useStorage } from "@vueuse/core";
import { Order } from "~/interfaces/order";

export const useTempOrder = () => {
    const storage = useStorage(
        "new-order-temp-item",
        new Order(""),
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
