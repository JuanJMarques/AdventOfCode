const std = @import("std");
const root = @import("root.zig");

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {
        std.log.err("{!}", .{err});
        return err;
    };
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u32 = 0;
    for (result.items) |line| {
        total += root.sumLine(line);
    }
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The addition of mul instructions is: {d}\n", .{total});
}