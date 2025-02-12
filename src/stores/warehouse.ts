import { defineStore } from "pinia";
import { apiWithTauri } from "~/api/tauri";
import { WareHouse } from "~/interfaces/warehouse";

export const useMyWarehouseStore = defineStore("myWarehouseStore", {
    state: () => ({ warehouses: [] as WareHouse[] }),
    actions: {
        async loadWareHouses() {
            if (!this.tauri) return;
            let warehouses = await apiWithTauri(
                this.tauri
            ).warehouse.getWarehouses();
            this.warehouses = [];
            for (const each of warehouses) {
                this.warehouses.push(WareHouse.fromInterface(each));
            }
        },
        async addWareHouse(wareHouse: WareHouse) {
            if (!this.tauri) return;
            await this.loadWareHouses();

            await apiWithTauri(this.tauri).warehouse.saveWarehouses([
                ...this.warehouses,
                wareHouse.toInterface(),
            ]);
            this.warehouses.push(wareHouse);
        },
    },
});
