export const useFlowbite = (callback: (flowbite: any) => any) => {
  if (import.meta.client) {
    import("flowbite").then((flowbite) => {
      callback(flowbite);
    });
  }
};
