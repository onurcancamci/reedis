import { Sizes, DataTypes, CommandTypes, ArgCounts } from "./adaptor";

export class Int {
    constructor(public val: number) {}
}

export class Path {
    constructor(public val: string[]) {}
}

export class Serializer {
    static NumSizeWrite(
        buf: Buffer,
        start: number,
        val: number | bigint,
        size: Sizes,
    ) {
        if (typeof val === "number") {
            switch (size) {
                case Sizes.u8:
                    return buf.writeUInt8(val, start);
                case Sizes.u16:
                    return buf.writeUInt16LE(val, start);
                case Sizes.u32:
                    return buf.writeUInt32LE(val, start);
                case Sizes.u64:
                    return buf.writeUIntLE(val, start, 6);
                case Sizes.usize:
                    return buf.writeUIntLE(val, start, 6);
            }
        } else {
            switch (size) {
                case Sizes.u8:
                    throw "Not Supported";
                case Sizes.u16:
                    throw "Not Supported";
                case Sizes.u32:
                    throw "Not Supported";
                case Sizes.u64:
                    return buf.writeBigUInt64LE(val, start);
                case Sizes.usize:
                    return buf.writeBigUInt64LE(val, start);
            }
        }
    }

    static TypeWrite(buf: Buffer, start: number, dt: DataTypes | CommandTypes) {
        buf.writeUInt16LE(dt, start);
    }

    static NumWrite(buf: Buffer, start: number, val: number) {
        Buffer.from(Float64Array.of(val).buffer).copy(buf, start);
    }
    static IntWrite(buf: Buffer, start: number, val: number) {
        buf.writeInt32LE(val, start);
    }
    static StrWrite(buf: Buffer, start: number, val: string) {
        buf.write(val, start, "utf-8");
    }
    static BuffWrite(buf: Buffer, start: number, val: Buffer) {
        val.copy(buf, start);
    }
    static BoolWrite(buf: Buffer, start: number, val: boolean) {
        buf.writeUInt8(val ? 1 : 0, start);
    }

    static SerializeCommand(comm: CommandTypes, ...args: any[]) {
        if (args.length != ArgCounts[comm]) {
            throw "Command Arg Count Doesnt Match";
        }
        let buf = Buffer.alloc(Sizes.usize + Sizes.u16);
        Serializer.TypeWrite(buf, Sizes.usize, comm);
        for (const arg of args) {
            buf = Buffer.concat([buf, Serializer.SerializeValue(arg)]);
        }
        Serializer.NumSizeWrite(buf, 0, buf.byteLength - 8, Sizes.usize);
        return buf;
    }

    static SerializeValue(val: any): Buffer {
        if (val instanceof Int) {
            const buf = Buffer.alloc(Sizes.usize + Sizes.u16 + Sizes.u64);
            Serializer.NumSizeWrite(buf, 0, Sizes.u16 + Sizes.u64, Sizes.usize);
            Serializer.TypeWrite(buf, Sizes.usize, DataTypes.Int64);
            Serializer.IntWrite(buf, Sizes.usize + Sizes.u16, val.val);
            return buf;
        } else if (val instanceof Path) {
            let buf = Buffer.alloc(Sizes.usize + Sizes.u16 + Sizes.u16);
            Serializer.TypeWrite(buf, Sizes.usize, DataTypes.Path);
            Serializer.NumSizeWrite(buf, 10, val.val.length, Sizes.u16);
            for (const p of val.val) {
                const pBuf = Buffer.from(p);
                const segmentBuf = Buffer.alloc(8 + pBuf.byteLength);
                Serializer.NumSizeWrite(
                    segmentBuf,
                    0,
                    pBuf.length,
                    Sizes.usize,
                );
                pBuf.copy(segmentBuf, 8);
                buf = Buffer.concat([buf, segmentBuf]);
            }
            Serializer.NumSizeWrite(buf, 0, buf.byteLength - 8, Sizes.usize);
            return buf;
        } else if (val === null) {
            const buf = Buffer.alloc(Sizes.usize + Sizes.u16);
            Serializer.NumSizeWrite(buf, 0, Sizes.u16, Sizes.usize);
            Serializer.TypeWrite(buf, Sizes.usize, DataTypes.Null);
            return buf;
        } else if (typeof val === "number") {
            const buf = Buffer.alloc(Sizes.usize + Sizes.u16 + Sizes.u64);
            Serializer.NumSizeWrite(buf, 0, Sizes.u16 + Sizes.u64, Sizes.usize);
            Serializer.TypeWrite(buf, Sizes.usize, DataTypes.Float64);
            Serializer.NumWrite(buf, Sizes.usize + Sizes.u16, val);
            return buf;
        } else if (typeof val === "boolean") {
            const buf = Buffer.alloc(Sizes.usize + Sizes.u16 + 1);
            Serializer.NumSizeWrite(buf, 0, Sizes.u16 + 1, Sizes.usize);
            Serializer.TypeWrite(buf, Sizes.usize, DataTypes.Bool);
            Serializer.BoolWrite(buf, Sizes.usize + Sizes.u16, val);
            return buf;
        } else if (typeof val === "string") {
            const valBuf = Buffer.from(val);
            const buf = Buffer.alloc(
                Sizes.usize + Sizes.u16 + valBuf.byteLength,
            );
            Serializer.NumSizeWrite(
                buf,
                0,
                Sizes.u16 + valBuf.byteLength,
                Sizes.usize,
            );
            Serializer.TypeWrite(buf, Sizes.usize, DataTypes.String);
            Serializer.BuffWrite(buf, Sizes.usize + Sizes.u16, valBuf);
            return buf;
        } else if (typeof val === "object" && Array.isArray(val)) {
            let header_buf = Buffer.alloc(8 + 2 + 4);
            Serializer.TypeWrite(header_buf, 8, DataTypes.Array);
            Serializer.NumSizeWrite(header_buf, 8 + 2, val.length, Sizes.u32);
            for (const v of val) {
                header_buf = Buffer.concat([
                    header_buf,
                    Serializer.SerializeValue(v),
                ]);
            }
            Serializer.NumSizeWrite(
                header_buf,
                0,
                header_buf.byteLength - 8,
                Sizes.usize,
            );
            return header_buf;
        } else {
            //table
            const keys = Object.keys(val);
            let header_buf = Buffer.alloc(8 + 2 + 4);
            Serializer.TypeWrite(header_buf, 8, DataTypes.Table);
            Serializer.NumSizeWrite(header_buf, 8 + 2, keys.length, Sizes.u32);
            for (const key of keys) {
                const str_key = Buffer.from(key);
                const str_key_buf = Buffer.alloc(8 + str_key.byteLength);
                Serializer.NumSizeWrite(
                    str_key_buf,
                    0,
                    str_key.byteLength,
                    Sizes.usize,
                );
                Serializer.BuffWrite(str_key_buf, 8, str_key);
                header_buf = Buffer.concat([
                    header_buf,
                    str_key_buf,
                    Serializer.SerializeValue(val[key]),
                ]);
            }
            Serializer.NumSizeWrite(
                header_buf,
                0,
                header_buf.byteLength - 8,
                Sizes.usize,
            );
            return header_buf;
        }
    }
}
