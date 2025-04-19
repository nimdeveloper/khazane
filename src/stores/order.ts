import { defineStore } from "pinia";
import { apiWithTauri } from "~/api/tauri";
import { Order, type IOrder } from "~/interfaces/order";

export const useMyOrderStore = defineStore("Order", {
    state: () => ({
        orders: [] as Order[],
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
                sorter: (a: IOrder, b: IOrder) => number;
            } | null;
        },
    }),

    getters: {
        filteredOrders: (state) => {
            const products = [];
            for (const item of state.orders) {
                if (
                    state.filters.status !== "all" &&
                    item.status !== state.filters.status
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
    },
    actions: {
        async loadOrders() {
            if (!this.tauri) return;
            let orders = await apiWithTauri().order.getOrders();
            this.orders = [];
            for (const each of orders) {
                this.orders.push(Order.fromInterface(each));
            }
        },
        async addOrder(order: Order, isNew = false) {
            if (!this.tauri) return;
            await this.loadOrders();
            if (isNew) {
                order.id = 0;
            } else {
                this.orders = this.orders.filter(
                    (each) => Number(each.id) !== Number(order.id)
                );
            }
            let res = await apiWithTauri().order.saveOrder(order.toInterface());
            let instance: Order | null = null;
            if (res) {
                instance = Order.fromInterface(res);
                this.orders.push(instance);
            }
            return instance;
        },
    },
});
