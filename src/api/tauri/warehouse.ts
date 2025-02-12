import type { IWareHouse } from "~/interfaces/warehouse";
import type { TauriStoreAccessor } from "~/types";

export default ($tauri: TauriStoreAccessor) => {
    return {
        async getWarehouses() {
            let res = await $tauri.get<IWareHouse[]>("warehouses");
            if (!res) return [];
            return res;
        },
        async saveWarehouses(products: IWareHouse[]) {
            return await $tauri.set("warehouses", products);
        },
    };
};
