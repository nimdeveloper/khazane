import { invoke } from "@tauri-apps/api/core";
import type { IMeasurementUnit } from "~/interfaces/measurement-unit";
import type { IProductCategory, IProductUnit } from "~/interfaces/product";
import type { TauriStoreAccessor } from "~/types";

export default ($tauri: TauriStoreAccessor) => {
    return {
        async getProducts() {
            try {
                let data = await invoke<IProductUnit[]>("get_products", {
                    filters: {},
                });
                return data;
            } catch (e) {
                console.error(e);
                return [];
            }
        },
        async saveProducts(products: IProductUnit[]) {
            return await $tauri.set("products", products);
        },
        async getMeasurementUnits() {
            let res = await $tauri.get<IMeasurementUnit[]>("measure_units");
            if (!res) return [];
            return res;
        },
        async saveMeasurementUnits(units: IMeasurementUnit[]) {
            return await $tauri.set("measure_units", units);
        },
        async getProductCategories() {
            try {
                let data = await invoke<IProductCategory[]>("get_categories", {
                    filters: {},
                });
                return data;
            } catch (e) {
                console.error(e);
                return [];
            }
        },
        async saveProductCategory(category: Omit<IProductCategory, "key">) {
            try {
                let data: any = await invoke<IProductCategory>("add_category", {
                    category: category,
                });
                data.id = normalizeId(data.id);
                return data;
            } catch (e) {
                console.error(e);
                return null;
            }
        },
    };
};
