import { invoke } from "@tauri-apps/api/core";
import type { IWareHouse } from "~/interfaces/warehouse";

export default () => {
    return {
        async getWarehouses() {
            try {
                let data = await invoke<IWareHouse[]>("list_warehouses", {
                    filters: {},
                });
                return data;
            } catch (e) {
                console.error(e);
                return [];
            }
        },
        async saveWareHouse(warehouse: IWareHouse) {
            try {
                let data = await invoke<IWareHouse>("create_warehouse", {
                    warehouse,
                });
                return data;
            } catch (e) {
                console.error(e);
                return null;
            }
        },
    };
};
