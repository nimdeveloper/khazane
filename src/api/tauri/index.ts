import products from "./product";
import type { TauriStoreAccessor } from "~/types";
import warehouse from "./warehouse";
import person from "./person";
import order from "./order";

export function apiWithTauri($tauri: TauriStoreAccessor) {
    return {
        products: products($tauri),
        warehouse: warehouse($tauri),
        person: person($tauri),
        order: order($tauri),
    };
}
