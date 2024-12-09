const std = @import("std");
const root = @import("root.zig");

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {
        std.log.err("{!}", .{err});
        return err;
    };
    const result = try root.divideLines(input);
    defer result.deinit();
    var safeReports: i32 = 0;
    const stdout = std.io.getStdOut().writer();
    for (result.items) |line| {
        const parsed = try root.parseLine(line);
        defer parsed.deinit();
        if (parsed.isSafe()) {
            safeReports += 1;
        }
    }
    try stdout.print("The number of safe reports is: {d}\n", .{safeReports});
}