import { invoke } from "@tauri-apps/api/core";
import type { IPerson } from "~/interfaces/person";

export default () => {
    return {
        async getPersons() {
            try {
                let data = await invoke<IPerson[]>("list_people", {
                    filters: {},
                });
                return data;
            } catch (e) {
                console.error(e);
                return [];
            }
        },
        async savePerson(person: IPerson) {
            try {
                let data = await invoke<IPerson>("create_person", {
                    person,
                });
                return data;
            } catch (e) {
                console.error(e);
                return null;
            }
        },
    };
};
