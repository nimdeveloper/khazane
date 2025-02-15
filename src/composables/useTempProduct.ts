import { ProductUnit } from "~/interfaces/product";
import { useStorage } from "@vueuse/core";

export const useTempProduct = () => {
    const storage = useStorage(
        "new-product-temp-item",
        new ProductUnit(""),
        sessionStorage,
        {
            serializer: {
                read: (v: any) => ProductUnit.read(v),
                write: (v: any) => (v ? v.write() : undefined),
            },
        }
    );
    return { storage };
};
