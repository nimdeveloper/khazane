import { defineStore } from "pinia";
import { apiWithTauri } from "~/api/tauri";
import { MeasurementUnit } from "~/interfaces/measurement-unit";

export const useMyMeasureStore = defineStore("myMeasureStore", {
    state: () => ({ units: [] as MeasurementUnit[] }),
    actions: {
        async loadUnits() {
            if (!this.tauri) return;
            let units = await apiWithTauri(
                this.tauri
            ).products.getMeasurementUnits();
            this.units = [];
            for (const each of units) {
                this.units.push(MeasurementUnit.fromInterface(each));
            }
        },
        async addUnit(unit: MeasurementUnit | string) {
            if (!(unit instanceof MeasurementUnit)) {
                unit = new MeasurementUnit(random(12), unit);
            }
            if (!this.tauri) return;
            await this.loadUnits();
            await apiWithTauri(this.tauri).products.saveMeasurementUnits([
                ...this.units,
                unit.toInterface(),
            ]);
        },
    },
});
