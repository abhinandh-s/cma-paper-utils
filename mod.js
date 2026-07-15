import { add, Greeter, paperIdToLevel } from "./lib/rs_lib.js";

export * from "./lib/rs_lib.js";

// adds
console.log(add(1, 1));

// greets
const greeter = new Greeter("world");
console.log(greeter.greet());
