import { defineStore } from "pinia";
import { apiWithTauri } from "~/api/tauri";
import { MeasurementUnit } from "~/interfaces/measurement-unit";

export const useMeasureStore = defineStore("Measure", {
    state: () => ({ units: [] as MeasurementUnit[] }),
    actions: {
        async loadUnits() {
            if (!this.tauri) return;
            let units = await apiWithTauri().products.getMeasurementUnits();
            this.units = [];
            for (const each of units) {
                this.units.push(MeasurementUnit.fromInterface(each));
            }
        },
        async addUnit(unit: MeasurementUnit | string) {
            if (!(unit instanceof MeasurementUnit)) {
                unit = new MeasurementUnit(0, unit);
            }
            let data = unit.toInterface();
            delete (data as any).id;
            let res = await apiWithTauri().products.saveMeasurementUnit(data);
            let instance: MeasurementUnit | null = null;
            if (res) {
                instance = MeasurementUnit.fromInterface(res);
                this.units.push(instance);
            }
            return instance;
        },
    },
});
