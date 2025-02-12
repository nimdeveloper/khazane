import type { Store } from "@tauri-apps/plugin-store";
import products from "./product";
import type { TauriStoreAccessor } from "~/types";
import warehouse from "./warehouse";

export function apiWithTauri($tauri: TauriStoreAccessor) {
    return {
        products: products($tauri),
        warehouse: warehouse($tauri),
    };
}
