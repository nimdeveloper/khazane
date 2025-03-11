export default (oldId: { tb: string; id: any }) => {
    if (oldId && oldId.tb) {
        if (oldId.id.String) {
            return `${oldId.tb}:${oldId.id.String}`;
        }
    }
    return oldId;
};
