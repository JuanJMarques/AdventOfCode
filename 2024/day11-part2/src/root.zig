const std = @import("std");
const testing = std.testing;
pub const BlinkKey = struct {stone: u64, blink: usize};
pub const BlinkCache = std.AutoHashMap(BlinkKey, u64);


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


pub fn blink(data: u64, count: usize, cache :*BlinkCache) u64 {
    const cachedValue = cache.get(BlinkKey{.stone = data, .blink = count});
    if (cachedValue) |value| {
        return value;
    }
    if (count == 0) {
        return 1;
    }
    var value: u64 = 0;
    if (data == 0){
        value = blink(1, count - 1, cache);
    }else {
        const digitLength = std.math.log10(data) + 1;
        if (@mod(digitLength, 2) == 0) {
            const middle = digitLength / 2;
            const divisor = std.math.pow(@TypeOf(data), 10, middle);
            const first = data / divisor;
            const second = @rem(data, divisor);
            value =  blink(first, count - 1, cache) + blink(second, count - 1, cache);
        }else {
            value =  blink(data * 2024, count - 1, cache);
        }
    }
    cache.put(BlinkKey{.stone = data, .blink = count}, value) catch unreachable;
    return value;
}

test "test case" {
    const input =
        \\125 17
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const blinkCount :usize = 25;
    const allocator = std.heap.page_allocator;
    var cache = BlinkCache.init(allocator);
    defer {
        cache.clearAndFree();
        cache.deinit();
    }
    for (result.items) |line| {
        const cleanLine  = std.mem.trim(u8, line, "\r\n");
        var arrangementIt = std.mem.splitSequence(u8, cleanLine, " ");
        while (arrangementIt.next()) |arrangementStr| {
            const arrangement = try std.fmt.parseInt(u64, arrangementStr, 10);
            total += blink(arrangement, blinkCount, &cache);
        }
    }

    try testing.expectEqual(55312, total);
}
