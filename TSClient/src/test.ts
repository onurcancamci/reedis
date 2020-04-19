import { Reedis } from "./adaptor";
import { Int, Path } from "./serializer";
import { inspect } from "util";

async function allTypes() {
    const reedis = new Reedis();
    console.log(await reedis.Set(["a", "b"], true));
    console.log(await reedis.Set(["a", "c"], 10.3));
    console.log(await reedis.Set(["a", "d"], { x: 1, y: true }));
    console.log(await reedis.Set(["a", "e"], new Int(5)));
    console.log(await reedis.Set(["a", "f"], new Path(["p1", "p2"])));
    console.log(await reedis.Set(["a", "g"], null));
    console.log(
        await reedis.Set(
            ["a", "h"],
            ["1", 2, new Int(3), true, false, null, { x: "y" }, [5, 6]],
        ),
    );

    console.log(inspect(await reedis.Get(["a"]), false, null, true));
}
