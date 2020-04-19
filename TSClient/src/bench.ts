import { Reedis } from "./adaptor";
import { Tedis } from "tedis";

async function main() {
    const useree = false;

    let reedis!: Reedis;
    let tedis!: Tedis;
    if (useree) {
        reedis = new Reedis();
    } else {
        tedis = new Tedis({
            port: 6379,
            host: "127.0.0.1",
        });
    }

    const to_set =
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const to_set2 = {
        a: 1,
        b: 2,
        c: {
            q: [1, 2, "asd"],
            w: {
                e: {
                    r: [
                        "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz",
                    ],
                },
            },
        },
    };
    const to_set3: any = {
        a: 1,
        b: 2,
        c: {
            q: [1, 2, "asd"],
            w: {
                e: {
                    r: [
                        "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz",
                    ],
                },
            },
        },
        d: {},
    };
    for (let k = 0; k < 1000; k++) {
        to_set3.d[k.toString()] = Math.random();
    }
    const to_set4 = JSON.stringify(to_set3);
    const start = Date.now();
    const ps: Promise<any>[] = [];
    let ct = 0;
    const it = 10000;
    setInterval(() => {
        console.log(`${((ct / it) * 100).toFixed(2)}%`);
    }, 5000);
    for (let k = 0; k < it; k++) {
        if (useree) {
            //await reedis.Set(k.toString(), to_set3);
            //ps.push(reedis.Set(k.toString(), to_set3));
            ps.push(
                (async () => {
                    ct++;
                    await reedis.Set(k.toString(), to_set3);
                })(),
            );
        } else {
            //await tedis.set(k.toString(), JSON.stringify(to_set3));
            ps.push(
                (async () => {
                    ct++;
                    await tedis.set(k.toString(), JSON.stringify(to_set3));
                })(),
            );
        }
    }
    await Promise.all(ps);
    const end = Date.now();
    console.log(`${((end - start) / 1000).toFixed(2)}s`);
    /*  */
    console.log("done");
}

// promise all 100000 to_set
// redis 7.7 sn, reedis 4.2 sn

// promise all 100000 to_set2
// redis 7.65 sn, reedis 6.53 sn
// redis 8.05 sn, reedis 6.82 sn

// await 100000 to_set3
// redis 4.21 sn, reedis 46.31 sn
