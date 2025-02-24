import type { IOrder } from "~/interfaces/order";
import type { TauriStoreAccessor } from "~/types";

export default ($tauri: TauriStoreAccessor) => {
    return {
        async getOrders() {
            let res = await $tauri.get<IOrder[]>("orders");
            if (!res) return [];
            return res;
        },
        async saveOrders(orders: IOrder[]) {
            return await $tauri.set("orders", orders);
        },
    };
};
