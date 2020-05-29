import { allTypes } from "./test";
import { Reedis } from "./adaptor";
async function main() {
    //await allTypes();
    const reedis = new Reedis();
    setInterval(async () => {
        await reedis.Set("a", 1);
        console.log(await reedis.Get("a"));
    }, 1000);
}

main().catch(console.log);
