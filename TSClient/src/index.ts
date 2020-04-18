import { writeFileSync } from "fs";
import { Serializer, Int, Path } from "./serializer";
import { Sizes, CommandTypes, Reedis } from "./adaptor";
import { Parser } from "./parser";
import { inspect } from "util";

async function main() {
    const reedis = new Reedis();
    console.log(await reedis.Set(["a", "b"], true));
    console.log(await reedis.Set(["a", "c"], 10.3));
    console.log(await reedis.Set(["a", "d"], { x: 1, y: true }));
    console.log(await reedis.Set(["a", "e"], new Int(5)));
    console.log(await reedis.Set(["a", "f"], new Path(["p1", "p2"])));
    console.log(await reedis.Set(["a", "g"], null));

    console.log(inspect(await reedis.Get(["a"]), false, null, true));
    console.log("done");
}

main().catch(console.log);
