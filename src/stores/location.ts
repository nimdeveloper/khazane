import { defineStore } from "pinia";
import { apiWithTauri } from "~/api/tauri";
import { Location } from "~/interfaces/location";

export const useLocationStore = defineStore("Location", {
    state: () => ({ locations: [] as Location[] }),
    actions: {
        async loadLocations() {
            if (!this.tauri) return;
            let locations = await apiWithTauri(
                this.tauri
            ).location.getLocations();
            this.locations = [];
            for (const each of locations) {
                this.locations.push(Location.fromInterface(each));
            }
        },
        async addLocation(location: Location) {
            if (!this.tauri) return;
            await this.loadLocations();

            await apiWithTauri(this.tauri).location.saveLocations([
                ...this.locations,
                location.toInterface(),
            ]);
            this.locations.push(location);
        },
    },
});
