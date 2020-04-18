import { writeFileSync } from "fs";
import { Serializer, Int, Path } from "./serializer";
import { Sizes, CommandTypes, Reedis } from "./adaptor";
import { Parser } from "./parser";

async function main() {
    //const reedis = new Reedis();
    //reedis.Set(["a", "b", "c"], true);
    console.log("done");
}

main().catch(console.log);
