const std = @import("std");
const root  = @import("root.zig");

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const blinkCount :usize = 75;
    const allocator = std.heap.page_allocator;
    var cache = root.BlinkCache.init(allocator);
    defer {
        cache.clearAndFree();
        cache.deinit();
    }
    for (result.items) |line| {
        const cleanLine  = std.mem.trim(u8, line, "\r\n");
        var arrangementIt = std.mem.splitSequence(u8, cleanLine, " ");
        while (arrangementIt.next()) |arrangementStr| {
            const arrangement = try std.fmt.parseInt(u64, arrangementStr, 10);
            total += root.blink(arrangement, blinkCount, &cache);
        }
    }
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The number of stones will you have after blinking 75 times is {d}\n", .{total});
}