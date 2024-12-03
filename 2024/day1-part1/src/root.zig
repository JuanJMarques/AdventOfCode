const std = @import("std");
const testing = std.testing;

pub const point = struct { a: i32, b: i32 };

pub const points = struct {
    left: std.ArrayList(i32),
    right: std.ArrayList(i32),

    pub fn deinit(self: points) void {
        self.left.deinit();
        self.right.deinit();
    }

    pub fn sort(self: *points) !void {
        const left = try self.left.toOwnedSlice();
        std.mem.sort(i32, left, {}, comptime std.sort.asc(i32));
        const oldLeft = self.left;
        defer oldLeft.deinit();
        self.left = std.ArrayList(i32).fromOwnedSlice(self.left.allocator, left);
        const right = try self.right.toOwnedSlice();
        std.mem.sort(i32, right, {}, comptime std.sort.asc(i32));
        const oldRight = self.right;
        defer oldRight.deinit();
        self.right = std.ArrayList(i32).fromOwnedSlice(self.right.allocator, right);
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

pub fn parseLine(line: []const u8) !point {
    var tokenizer = std.mem.tokenize(u8, line, " ");
    const part1 = tokenizer.next().?;
    const part2 = tokenizer.next().?;

    // Convierte los tokens a números
    const a = try std.fmt.parseInt(i32, part1, 10);
    const b = try std.fmt.parseInt(i32, part2, 10);

    return .{ .a = a, .b = b }; // Devuelve los números como una estructura
}

pub fn transformLists(list: std.ArrayList(point)) !points {
    const allocator = std.heap.page_allocator;
    var converted = points{
        .left = try std.ArrayList(i32).initCapacity(allocator, list.capacity),
        .right = try std.ArrayList(i32).initCapacity(allocator, list.capacity),
    };
    const leftPtr = &converted.left;
    const rightPtr = &converted.right;

    for (list.items) |item| {
        try leftPtr.append(item.a);
        try rightPtr.append(item.b);
    }
    return converted;
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
    for (result.items) |line| {
        const parsed = try parseLine(line);
        try testing.expect(parsed.a>=1 and parsed.a<=4 and parsed.b>=3 and parsed.b<=9);
        try parsedLines.append(parsed);
    }

    var converted = try transformLists(parsedLines);
    defer converted.deinit();
    try converted.sort();
    var i : usize = 0;
    var total : i32 = 0;
    while (i < converted.left.capacity) : (i += 1) {
        const lei = converted.left.items[i];
        const rii = converted.right.items[i];
        const dif  =
            if (lei>rii)
                lei - rii
            else
                rii - lei
        ;
        total += dif;
    }
    try testing.expectEqual(11, total);
}