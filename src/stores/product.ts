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
        } as {
            status: string;
            sort: {
                prefix: string;
                label: string;
                value: string;
                sorter: (a: IProductUnit, b: IProductUnit) => number;
            } | null;
            category: ProductCategory | null;
            warehouse: WareHouse | null;
        },
    }),
    getters: {
        filteredProducts: (state) => {
            const products = [];
            for (const item of state.products) {
                if (
                    state.filters.status !== "all" &&
                    item.status !== state.filters.status
                ) {
                    continue;
                }
                if (
                    state.filters.category &&
                    state.filters.category.key &&
                    item.category?.key !== state.filters.category.key
                ) {
                    continue;
                }
                if (
                    state.filters.warehouse &&
                    state.filters.warehouse.key &&
                    item.ware_houses.filter(
                        (each) =>
                            each.warehouse?.key === state.filters.warehouse?.key
                    ).length < 1
                ) {
                    continue;
                }
                products.push(item);
            }
            if (state.filters.sort) {
                return products.sort(state.filters.sort.sorter);
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
        async loadProducts() {
            if (!this.tauri) return;
            let products = await apiWithTauri(
                this.tauri
            ).products.getProducts();
            this.products = [];
            for (const each of products) {
                this.products.push(ProductUnit.fromInterface(each));
            }
        },
        async addProduct(product: ProductUnit) {
            if (!this.tauri) return;
            await this.loadProducts();
            await apiWithTauri(this.tauri).products.saveProducts([
                ...this.products,
                product.toInterface(),
            ]);
            this.products.push(product);
        },
        async addCategory(label: string) {
            if (!this.tauri) return;
            await this.loadCategories();
            const category = new ProductCategory(random(12), label);
            await apiWithTauri(this.tauri).products.saveProductCategories([
                ...this.categories,
                category.toInterface(),
            ]);
            this.categories.push(category);
        },
        async loadCategories() {
            if (!this.tauri) return;
            let categories = await apiWithTauri(
                this.tauri
            ).products.getProductCategories();
            this.categories = [];
            for (const each of categories) {
                this.categories.push(ProductCategory.fromInterface(each));
            }
        },
    },
});
