import { invoke } from "@tauri-apps/api/core";
import type { ILocation } from "~/interfaces/location";

export default () => {
    return {
        async getLocations() {
            try {
                let data = await invoke<ILocation[]>("list_locations", {
                    filters: {},
                });
                return data;
            } catch (e) {
                console.error(e);
                return [];
            }
        },
        async saveLocation(location: ILocation) {
            try {
                let data = await invoke<ILocation>("create_location", {
                    location,
                });
                return data;
            } catch (e) {
                console.error(e);
                return null;
            }
        },
    };
};
