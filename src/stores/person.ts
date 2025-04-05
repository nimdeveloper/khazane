import { defineStore } from "pinia";
import { apiWithTauri } from "~/api/tauri";
import { Person, type IPerson } from "~/interfaces/person";

export const useMyPersonStore = defineStore("myPersonStore", {
    state: () => ({ persons: [] as Person[] }),
    actions: {
        async loadPersons() {
            if (!this.tauri) return;
            let persons = await apiWithTauri().person.getPersons();
            this.persons = [];
            for (const each of persons) {
                this.persons.push(Person.fromInterface(each));
            }
        },
        async addPerson(person: Person) {
            if (!this.tauri) return;
            await this.loadPersons();

            let res = await apiWithTauri().person.savePerson(
                person.toInterface()
            );
            let instance: Person | null = null;
            if (res) {
                instance = Person.fromInterface(res);
                this.persons.push(instance);
            }
            return instance;
        },
    },
});
