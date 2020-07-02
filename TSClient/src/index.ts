import { allTypes } from "./test";
import { Reedis } from "./adaptor";
import { Parser } from "./parser";
async function main() {
    //await allTypes();
    const reedis = new Reedis();
    const ev = await reedis.readPromise;
    const com = Parser.ParseCommand(ev);
    console.log(com);
}

main().catch(console.log);
