import { defineStore } from "pinia";

interface GlobalState {
  SidebarOpen: boolean;
  HasFilter: boolean;
}
export const useMyGlobalStore = defineStore("myGlobalStore", {
  state: (): GlobalState => ({ SidebarOpen: false, HasFilter: false }),

  actions: {
    toggleSidebar() {
      this.SidebarOpen = !this.SidebarOpen;
    },
    noFilter() {
      this.HasFilter = false;
    },
    withFilter() {
      this.HasFilter = true;
    },
  },
});
