import "pinia";
import type { Router } from "vue-router";
import { Store as TauriStore } from "@tauri-apps/plugin-store";

declare module "pinia" {
    export interface PiniaCustomProperties {
        tauri: TauriStoreAccessor;
    }
}
export interface TauriStoreAccessor extends Pick<TauriStore, "get" | "set"> {}
