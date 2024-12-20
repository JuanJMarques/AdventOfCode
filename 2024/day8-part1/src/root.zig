const std = @import("std");
const testing = std.testing;

pub const location = struct {
    x: i32,
    y: i32,

    pub fn add(l1: location, l2: location) location {
        return location{ .x = l1.x + l2.x, .y = l1.y + l2.y };
    }

    pub fn sub(l1: location, l2: location) location {
        return location{ .x = l1.x - l2.x, .y = l1.y - l2.y };
    }

    pub fn validate(self: location, minX: i32, minY: i32, maxX: i32, maxY: i32) bool {
        return self.x >= minX and self.x < maxX and self.y >= minY and self.y < maxY;
    }
};

pub fn divideLines(input: []const u8) !std.ArrayList([]const u8) {
    const allocator = std.heap.page_allocator;
    var lineList = std.ArrayList([]const u8).init(allocator);
    var lines = std.mem.splitSequence(u8, input, "\n");
    while (lines.next()) |line| {
        try lineList.append(std.mem.trim(u8, line, "\r\n"));
    }
    return lineList;
}

pub fn readFile(path: []const u8) ![]const u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return try file.readToEndAlloc(std.heap.page_allocator, @as(usize, std.math.maxInt(usize)));
}

test "test case" {
    const input =
        \\............
        \\........0...
        \\.....0......
        \\.......0....
        \\....0.......
        \\......A.....
        \\............
        \\............
        \\........A...
        \\.........A..
        \\............
        \\............
    ;

    const result = try divideLines(input);
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
    try testing.expectEqual(14, total);
}