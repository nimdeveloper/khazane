import type { ILocation } from "~/interfaces/location";
import type { TauriStoreAccessor } from "~/types";

export default ($tauri: TauriStoreAccessor) => {
    return {
        async getLocations() {
            let res = await $tauri.get<ILocation[]>("locations");
            if (!res) return [];
            return res;
        },
        async saveLocations(locations: ILocation[]) {
            return await $tauri.set("locations", locations);
        },
    };
};
