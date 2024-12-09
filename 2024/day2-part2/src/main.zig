const std = @import("std");
const root = @import("root.zig");

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {
        std.log.err("{!}", .{err});
        return err;
    };
    const result = try root.divideLines(input);
    defer result.deinit();
    var safeReports2: i32 = 0;
    const stdout = std.io.getStdOut().writer();
    for (result.items) |line| {
        const parsed = try root.parseLine(line);
        defer parsed.deinit();
        if (parsed.isSafe2(-1)) {
            safeReports2 += 1;
        }
    }
    try stdout.print("The number of safe reports with 1 toleration is: {d}\n", .{safeReports2});
}