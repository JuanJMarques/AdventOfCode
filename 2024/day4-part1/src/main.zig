const std = @import("std");
const root  = @import("root.zig");

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u32 = 0;
    for (result.items, 0..) |_, i| {
        for (result.items[i], 0..) |_, j| {
            total += root.searchForXMAS(result.items, j, i, 0, null);
        }
    }
    const stdout = std.io.getStdOut().writer();
    try stdout.print("XMAS appears {d} times\n", .{total});
}