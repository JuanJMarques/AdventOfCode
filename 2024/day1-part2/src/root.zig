const std = @import("std");
const testing = std.testing;

pub const point = struct { a: i32, b: i32 };

pub fn divideLines(input: []const u8) !std.ArrayList([]const u8) {
    const allocator = std.heap.page_allocator;
    var lineList = std.ArrayList([]const u8).init(allocator);
    var lines = std.mem.splitSequence(u8, input, "\n");
    while (lines.next()) |line| {
        try lineList.append(std.mem.trim(u8, line, "\r\n"));
    }
    return lineList;
}

pub fn parseLine(line: []const u8) !point {
    var tokenizer = std.mem.tokenize(u8, line, " ");
    const part1 = tokenizer.next().?;
    const part2 = tokenizer.next().?;

    // Convierte los tokens a números
    const a = try std.fmt.parseInt(i32, part1, 10);
    const b = try std.fmt.parseInt(i32, part2, 10);

    return .{ .a = a, .b = b }; // Devuelve los números como una estructura
}

pub fn readFile(path: []const u8) ![] const u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return try file.readToEndAlloc(std.heap.page_allocator, @as(usize, std.math.maxInt(usize)));
}

test "test case" {

    const input =
        \\3   4
        \\4   3
        \\2   5
        \\1   3
        \\3   9
        \\3   3
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var parsedLines = std.ArrayList(point).init(std.heap.page_allocator);
    defer parsedLines.deinit();
    var keys = try std.ArrayList(i32).initCapacity(std.heap.page_allocator, parsedLines.capacity);
    defer keys.deinit();
    var values = std.AutoHashMap(i32, i32).init(std.heap.page_allocator);
    defer values.deinit();
    for (result.items) |line| {
        const parsed = try parseLine(line);
        try keys.append(parsed.a);
        try values.put(parsed.b, if (!values.contains(parsed.b)) 1 else values.get(parsed.b).? + 1);
    }

    var total: i32 = 0;
    for(keys.items) |key| {
        if (values.contains(key)) {
            total += key * values.get(key).?;
        }
    }
    try testing.expectEqual(31, total);
}