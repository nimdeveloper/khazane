import type { IMeasurementUnit } from "~/interfaces/measurement-unit";
import type { IProductCategory, IProductUnit } from "~/interfaces/product";
import type { TauriStoreAccessor } from "~/types";

export default ($tauri: TauriStoreAccessor) => {
    return {
        async getProducts() {
            let res = await $tauri.get<IProductUnit[]>("products");
            if (!res) return [];
            return res;
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
            let res = await $tauri.get<IProductCategory[]>(
                "product_categories"
            );
            if (!res) return [];
            return res;
        },
        async saveProductCategories(categories: IProductCategory[]) {
            return await $tauri.set("product_categories", categories);
        },
    };
};
