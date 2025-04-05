import products from "./product";
import warehouse from "./warehouse";
import person from "./person";
import order from "./order";
import location from "./location";

export function apiWithTauri() {
    return {
        products: products(),
        warehouse: warehouse(),
        person: person(),
        order: order(),
        location: location(),
    };
}
