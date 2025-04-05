import { defineStore } from "pinia";
import { apiWithTauri } from "~/api/tauri";
import { WareHouse } from "~/interfaces/warehouse";

export const useMyWarehouseStore = defineStore("myWarehouseStore", {
    state: () => ({ warehouses: [] as WareHouse[] }),
    actions: {
        async loadWareHouses() {
            let warehouses = await apiWithTauri().warehouse.getWarehouses();
            this.warehouses = [];
            for (const each of warehouses) {
                this.warehouses.push(WareHouse.fromInterface(each));
            }
        },
        async addWareHouse(wareHouse: WareHouse) {
            await this.loadWareHouses();

            let res = await apiWithTauri().warehouse.saveWareHouse(
                wareHouse.toInterface()
            );
            let instance = null;
            if (res) {
                instance = WareHouse.fromInterface(res);
                this.warehouses.push(instance);
            }
            return instance;
        },
    },
});
