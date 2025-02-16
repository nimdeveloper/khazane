import type { IPerson } from "~/interfaces/person";
import type { TauriStoreAccessor } from "~/types";

export default ($tauri: TauriStoreAccessor) => {
    return {
        async getPersons() {
            let res = await $tauri.get<IPerson[]>("persons");
            if (!res) return [];
            return res;
        },
        async savePersons(persons: IPerson[]) {
            return await $tauri.set("persons", persons);
        },
    };
};
