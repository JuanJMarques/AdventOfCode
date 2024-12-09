const std = @import("std");
const testing = std.testing;

pub const report = struct {
    levels: std.ArrayList(i32),

    pub fn isSafe(self: report) bool {
        var safe = true;
        const levels = self.levels.items;
        if (levels.len <= 1) {
            return safe;
        }
        var i: u32 = 1;
        var last = levels[0];
        var inc = true;
        var dec = true;
        while (safe and i < levels.len) {
            const act = levels[i];
            inc = inc and last < act;
            dec = dec and last > act;
            const diff: u32 = @abs(last - act);
            safe = (inc or dec) and diff >= 1 and diff <= 3;
            last = act;
            i += 1;
        }
        return safe;
    }

    pub fn isSafe2(self: report, toIgnore: i32) bool {
        var safe = true;
        const levels = self.levels.items;
        if (levels.len <= 1) {
            return safe;
        }
        var i: i32 = 1;
        var last = levels[0];
        if (toIgnore == 0) {
            last = levels[1];
            i = 2;
        }
        var inc = true;
        var dec = true;
        while (safe and i < levels.len) {
            if (i != toIgnore) {
                const act = levels[@abs(i)];
                inc = inc and last < act;
                dec = dec and last > act;
                const diff: u32 = @abs(last - act);
                safe = (inc or dec) and diff >= 1 and diff <= 3;
                last = act;
            }
            i += 1;
        }
        var safe2 = false;
        if (!safe and toIgnore < 0) {
            var j: i32 = 0;
            while (!safe2 and j < i) {
                safe2 = safe2 or self.isSafe2(j);
                j += 1;
            }
        }
        return safe or safe2;
    }

    pub fn deinit(self: report) void {
        self.levels.deinit();
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

pub fn parseLine(line: []const u8) !report {
    const allocator = std.heap.page_allocator;
    var tokenizer = std.mem.tokenize(u8, line, " ");
    var lineReport = report{ .levels = std.ArrayList(i32).init(allocator) };
    while (tokenizer.next()) |lelvel| {
        const levelValue = try std.fmt.parseUnsigned(i32, lelvel, 10);
        try lineReport.levels.append(levelValue);
    }
    return lineReport;
}

pub fn readFile(path: []const u8) ![]const u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return try file.readToEndAlloc(std.heap.page_allocator, @as(usize, std.math.maxInt(usize)));
}

test "test case" {
    const input =
        \\7 6 4 2 1
        \\1 2 7 8 9
        \\9 7 6 2 1
        \\1 3 2 4 5
        \\8 6 4 4 1
        \\1 3 6 7 9
    ;

    const result = try divideLines(input);
    defer result.deinit();
    var safeReports: i32 = 0;
    var safeReports2: i32 = 0;
    for (result.items) |line| {
        const parsed = try parseLine(line);
        defer parsed.deinit();
        if (parsed.isSafe()) {
            safeReports += 1;
        }
        if (parsed.isSafe2(-1)) {
            safeReports2 += 1;
        }
    }
    // try testing.expectEqual(2, safeReports);
    try testing.expectEqual(4, safeReports2);
}