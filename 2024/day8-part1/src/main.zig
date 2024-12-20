const std = @import("std");
const root  = @import("root.zig");
const location = root.location;

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const allocator = std.heap.page_allocator;
    var antennaMap = std.AutoHashMap(u8, std.ArrayList(location)).init(allocator);
    defer {
        var it = antennaMap.keyIterator();
        while (it.next()) |c| {
            var list = antennaMap.get(c.*).?;
            list.deinit();
        }
        antennaMap.clearAndFree();
        antennaMap.deinit();
    }
    const maxY : i32 = @intCast(result.items.len);
    const maxX : i32 = @intCast(result.items[0].len);
    for (result.items, 0..) |line, y| {
        const cleanline = std.mem.trim(u8, line, "\r\n");
        for (cleanline, 0..) |c, x| {
            if (c != '.') {
                if (!antennaMap.contains(c)) {
                    try antennaMap.put(c, std.ArrayList(location).init(allocator));
                }
                var antennaList = antennaMap.get(c).?;
                try antennaList.append(location{ .x = @intCast(x), .y = @intCast(y) });
                try antennaMap.put(c, antennaList);
            }
        }
    }
    var antinodeSet = std.AutoHashMap(location, void).init(allocator);
    defer antinodeSet.deinit();
    var antennaIt = antennaMap.keyIterator();
    while (antennaIt.next()) |antenna| {
        const antennaLocations = antennaMap.get(antenna.*).?.items;
        if (antennaLocations.len >= 2) {
            var i: usize = 0;
            while (i < antennaLocations.len - 1) {
                const antenna1 = antennaLocations[i];
                var j = i + 1;
                i += 1;
                while (j < antennaLocations.len) {
                    const antenna2 = antennaLocations[j];
                    const verctor = location.sub(antenna2, antenna1);
                    const antiNode1 = location.add(antenna2, verctor);
                    if (antiNode1.validate(0, 0, maxX, maxY)) {
                        try antinodeSet.put(antiNode1, {});
                    }
                    const antiNode2 = location.sub(antenna1, verctor);
                    if (antiNode2.validate(0, 0, maxX, maxY)) {
                        try antinodeSet.put(antiNode2, {});
                    }
                    j += 1;
                }
            }
        }
    }
    total = antinodeSet.count();
    const stdout = std.io.getStdOut().writer();
    try stdout.print("There are {d} locations with an anti node\n", .{total});
}