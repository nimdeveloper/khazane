import { invoke } from "@tauri-apps/api/core";
import type { IOrder } from "~/interfaces/order";

export default () => {
    return {
        async getOrders() {
            try {
                let data = await invoke<IOrder[]>("list_orders", {
                    filters: {},
                });
                return data;
            } catch (e) {
                console.error(e);
                return [];
            }
        },
        async saveOrder(order: IOrder) {
            try {
                let data = await invoke<IOrder>("create_order", {
                    order,
                });
                return data;
            } catch (e) {
                console.error(e);
                return null;
            }
        },
    };
};
