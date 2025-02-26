import { defineStore } from "pinia";
import { apiWithTauri } from "~/api/tauri";
import { Person, type IPerson } from "~/interfaces/person";

export const useMyPersonStore = defineStore("myPersonStore", {
    state: () => ({ persons: [] as Person[] }),
    actions: {
        async loadPersons() {
            if (!this.tauri) return;
            let persons = await apiWithTauri(this.tauri).person.getPersons();
            this.persons = [];
            for (const each of persons) {
                this.persons.push(Person.fromInterface(each));
            }
        },
        async addPerson(person: Person) {
            if (!this.tauri) return;
            await this.loadPersons();

            await apiWithTauri(this.tauri).person.savePersons([
                ...this.persons,
                person.toInterface(),
            ]);
            this.persons.push(person);
        },
    },
});
