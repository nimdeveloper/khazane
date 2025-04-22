import { defineStore } from "pinia";
import { apiWithTauri } from "~/api/tauri";
import {
    ProductCategory,
    ProductUnit,
    type IProductUnit,
} from "~/interfaces/product";
import type { WareHouse } from "~/interfaces/warehouse";

export const useMyProductStore = defineStore("myProductStore", {
    state: () => ({
        products: [] as ProductUnit[],
        categories: [] as ProductCategory[],
        filters: {
            status: "all",
            sort: null,
            category: null,
            warehouse: null,
            search: "",
        } as {
            status: string;
            sort: {
                prefix: string;
                label: string;
                key: string;
                sort_by: string;
                direction: string;
            } | null;
            category: ProductCategory | null;
            warehouse: WareHouse | null;
            search: string;
        },
    }),
    getters: {
        filteredProducts: (state) => {
            const products: ProductUnit[] = [];
            for (const item of state.products) {
                if (
                    state.filters.status !== "all" &&
                    item.status !== state.filters.status
                ) {
                    continue;
                }
                if (
                    state.filters.category &&
                    state.filters.category.id &&
                    item.category?.id !== state.filters.category.id
                ) {
                    continue;
                }
                if (
                    state.filters.warehouse &&
                    state.filters.warehouse.id &&
                    item.ware_houses.filter(
                        (each) =>
                            each.warehouse?.id === state.filters.warehouse?.id
                    ).length < 1
                ) {
                    continue;
                }
                products.push(item);
            }
            return products;
        },
        productsCount(state) {
            return {
                all: state.products.length,
                active: state.products.filter(
                    (item) => item.status === "active"
                ).length,
                draft: state.products.filter((item) => item.status === "draft")
                    .length,
                inactive: state.products.filter(
                    (item) => item.status === "inactive"
                ).length,
            };
        },
    },
    actions: {
        // Product
        async loadProducts() {
            if (!this.tauri) return;
            this.filters.sort;
            let products = await apiWithTauri().products.getProducts({
                ...(this.filters.sort
                    ? {
                          sort_by: this.filters.sort.sort_by,
                          sort_order: this.filters.sort.direction,
                      }
                    : {}),
                ...(this.filters.status &&
                this.filters.status.toLowerCase() !== "all"
                    ? {
                          status: this.filters.status,
                      }
                    : {}),
                ...(this.filters.category
                    ? { category_id: this.filters.category.id }
                    : {}),
                // TODO: Uncomment
                // ...(this.filters.warehouse
                //     ? { warehouse_id: this.filters.warehouse.id }
                //     : {}),
                ...(this.filters.search
                    ? { search_term: this.filters.search }
                    : {}),
            });
            this.products = [];
            for (const each of products) {
                this.products.push(ProductUnit.fromInterface(each));
            }
        },
        async addProduct(product: ProductUnit) {
            await this.loadProducts();
            let new_product = await apiWithTauri().products.saveProduct(
                product
            );
            let instance: ProductUnit | null = null;
            if (new_product) {
                instance = ProductUnit.fromInterface(new_product);
                this.products.push();
            }
            return instance;
        },
        // Category
        async addCategory(label: string) {
            if (!this.tauri) return;
            const category = new ProductCategory(0, label);
            let data = category.toInterface();
            delete (data as any).id;
            let res = await apiWithTauri().products.saveProductCategory(data);
            let instance: ProductCategory | null = null;
            if (res) {
                instance = ProductCategory.fromInterface(res);
                this.categories.push(instance);
            }
            return instance;
        },
        async loadCategories() {
            if (!this.tauri) return;
            let categories =
                await apiWithTauri().products.getProductCategories();
            this.categories = [];
            for (const each of categories) {
                this.categories.push(ProductCategory.fromInterface(each));
            }
        },
    },
});
