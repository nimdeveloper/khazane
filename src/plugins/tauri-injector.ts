import type { PiniaPluginContext } from "pinia";
import { Store } from "@tauri-apps/plugin-store";

async function myPiniaPlugin({ store }: PiniaPluginContext) {
    // console.log("HERE", store);
    // your plugin logic here
    // Create a new store or load the existing one,
    // note that the options will be ignored if a `Store` with that path has already been created
    const data_store = await Store.load("store.json", { autoSave: 1000 });

    store.tauri = {
        async get(key) {
            return await data_store.get(key);
        },
        async set(key: string, value: any) {
            return await data_store.set(key, value);
        },
    };
    store.$subscribe((...args) => {
        // console.log(args);
    });
}

export default defineNuxtPlugin(({ $pinia }: any) => {
    $pinia.use(myPiniaPlugin);
});
