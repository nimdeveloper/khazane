import { invoke } from "@tauri-apps/api/core";
import type { IMeasurementUnit } from "~/interfaces/measurement-unit";
import type { IProductCategory, IProductUnit } from "~/interfaces/product";

export default () => {
    return {
        async getProducts(filters = {}) {
            try {
                let data = await invoke<IProductUnit[]>("get_products", {
                    filters,
                });
                return data;
            } catch (e) {
                console.error(e);
                return [];
            }
        },
        async saveProduct(product: IProductUnit) {
            try {
                let data = await invoke<IProductUnit>("add_product", {
                    product,
                });
                return data;
            } catch (e) {
                console.error(e);
                return null;
            }
        },
        async getMeasurementUnits() {
            try {
                let data = await invoke<IMeasurementUnit[]>(
                    "get_measure_units",
                    {
                        filters: {},
                    }
                );
                return data;
            } catch (e) {
                console.error(e);
                return [];
            }
        },
        async saveMeasurementUnit(unit: IMeasurementUnit) {
            try {
                let data = await invoke<IMeasurementUnit>(
                    "create_measure_unit",
                    {
                        unit,
                    }
                );
                return data;
            } catch (e) {
                console.error(e);
                return null;
            }
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
                let data: any = await invoke<IProductCategory>(
                    "create_category",
                    {
                        category: category,
                    }
                );
                // data.id = normalizeId(data.id);
                return data;
            } catch (e) {
                console.error(e);
                return null;
            }
        },
    };
};
