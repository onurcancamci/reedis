import { DataTypes, Sizes, ArgCounts, CommandTypes } from "./adaptor";
import { Path, Int } from "./serializer";

export class Parser {
    static ParseCommand(buf: Buffer) {
        //size included
        const type: CommandTypes = buf.readInt16LE(8);
        const args: any[] = [];
        let ind = 10;
        for (let k = 0; k < ArgCounts[type]; k++) {
            const arg_len = buf.readUInt32LE(ind);
            const arg_val = Parser.ParseValue(
                buf.slice(ind, ind + 8 + arg_len),
            );
            args.push(arg_val);
            ind += 8 + arg_len;
        }
        return {
            command: type,
            args,
        };
    }
    static ParseValue(buf: Buffer, native: boolean = true) {
        //size included
        const type = buf.readUInt16LE(8);
        switch (type) {
            case DataTypes.Bool:
                return !!buf[10];
            case DataTypes.Float64:
                const ab = new ArrayBuffer(8);
                const ub = new Uint8Array(ab);
                buf.copy(ub, 0, 10);
                const fab = new Float64Array(ab);
                return fab[0];
            case DataTypes.Int64:
                return native
                    ? buf.readInt32LE(10)
                    : new Int(buf.readInt32LE(10));
            case DataTypes.Null:
                return null;
            case DataTypes.Path:
                const pathRaw: string[] = [];
                const segment_count = buf.readUInt16LE(10);
                let indp = 12;
                for (let k = 0; k < segment_count; k++) {
                    const selen = buf.readUInt32LE(indp); //usize in reality
                    indp += 8;
                    const segment = buf
                        .slice(indp, indp + selen)
                        .toString("utf8");
                    indp += selen;
                    pathRaw.push(segment);
                }
                return native ? pathRaw : new Path(pathRaw);
            case DataTypes.String:
                return buf.slice(10).toString("utf8");
            case DataTypes.Table:
                const obj: Record<string, any> = {};
                const kv_count = buf.readUInt32LE(10);
                let ind = 10 + Sizes.u32;
                for (let k = 0; k < kv_count; k++) {
                    const slen = buf.readUInt32LE(ind); //usize in reality
                    ind += 8;
                    const key = buf.slice(ind, ind + slen).toString("utf8");
                    ind += slen;
                    const val_size = buf.readUInt32LE(ind);
                    const val = Parser.ParseValue(
                        buf.slice(ind, ind + val_size + 8),
                    );
                    ind += 8 + val_size;
                    obj[key] = val;
                }
                return obj;
        }
    }
}
