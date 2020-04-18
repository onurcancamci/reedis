import * as net from "net";
import { Serializer, Path } from "./serializer";
import { Parser } from "./parser";

export enum Sizes {
    u8 = 1,
    u16 = 2,
    u32 = 4,
    u64 = 8,
    usize = 8,
}

export enum DataTypes {
    Null = 0,
    String = 1,
    Int64 = 2,
    Float64 = 3,
    Path = 4,
    Table = 5,
    Bool = 6,
}

export enum CommandTypes {
    Get = 1,
    Set = 2,
    Result = 3,
    Terminate = 65535,
}
export const ArgCounts = {
    [CommandTypes.Get]: 1,
    [CommandTypes.Set]: 2,
    [CommandTypes.Result]: 1,
    [CommandTypes.Terminate]: 0,
};

export class Reedis {
    socket: net.Socket;
    connectionPromise: Promise<any>;
    readPromise: Promise<Buffer>;
    readResolve!: Function;

    tmpReadBuf: Buffer = Buffer.alloc(0);
    tmpReadStarted: boolean = false;

    async Set(path: string | string[] | Path, val: any) {
        await this.connectionPromise;
        if (typeof path !== "string" && !(path instanceof Path)) {
            path = new Path(path);
        }
        const buf = Serializer.SerializeCommand(CommandTypes.Set, path, val);
        return Parser.ParseCommand(await this.Exec(buf));
    }

    async Get(path: string | string[] | Path) {
        await this.connectionPromise;
        if (typeof path !== "string" && !(path instanceof Path)) {
            path = new Path(path);
        }
        const buf = Serializer.SerializeCommand(CommandTypes.Get, path);
        return Parser.ParseCommand(await this.Exec(buf));
    }

    private async Exec(buf: Buffer) {
        this.socket.write(buf);
        const result = await this.readPromise;
        return result;
    }
    private async ReadDone(buf: Buffer) {
        this.tmpReadBuf = Buffer.alloc(0);
        this.tmpReadStarted = false;
        this.readResolve(buf);
        this.readPromise = new Promise((res) => {
            this.readResolve = res;
        });
    }
    private async Read(buf: Buffer) {
        //console.log(buf);
        if (this.tmpReadStarted) {
            this.tmpReadBuf = Buffer.concat([this.tmpReadBuf, buf]);
            buf = this.tmpReadBuf;
        }
        let len = buf.readUInt32LE(0);
        if (buf.length - 8 === len) {
            //full packet
            return this.ReadDone(buf);
        } else if (buf.length - 8 > len) {
            //more byte than neeeded
            let ind = 0;
            do {
                len = buf.readUInt32LE(ind);
                this.ReadDone(buf.slice(ind, ind + len + 8));
                ind += len + 8;
            } while (ind + len + 8 <= buf.length);
            //now there is less byte than needed
            if (buf.length - ind > 0) {
                // there is still data
                this.tmpReadStarted = true;
                this.tmpReadBuf = buf.slice(ind);
            }
        } else {
            this.tmpReadStarted = true;
            this.tmpReadBuf = buf;
        }
    }

    constructor(socket: net.Socket);
    constructor(port?: number, host?: string);
    constructor(port: number | net.Socket = 7071, host: string = "localhost") {
        if (typeof port === "number") {
            this.socket = new net.Socket();
            this.connectionPromise = new Promise((res, rej) => {
                this.socket.connect(port, host, (err?: any) =>
                    err ? rej(err) : res(),
                );
            });
        } else {
            this.socket = port;
            this.connectionPromise = new Promise((res, _) => {
                res();
            });
        }
        this.socket.on("data", this.Read.bind(this));
        this.socket.on("close", () => {
            console.log("Socket Closed");
        });
        this.readPromise = new Promise((res) => {
            this.readResolve = res;
        });
        process.on("exit", () => {
            this.socket.end();
        });
    }
}
