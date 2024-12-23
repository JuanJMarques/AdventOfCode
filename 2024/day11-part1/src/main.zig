const std = @import("std");
const root  = @import("root.zig");

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u32 = 0;
    const allocator = std.heap.page_allocator;
    const blinkCount :usize = 25;
    for (result.items) |line| {
        const cleanLine  = std.mem.trim(u8, line, "\r\n");
        var arragementList = try root.parseLine(cleanLine);
        defer {
            while (arragementList.popFirst()) |node| {
                allocator.destroy(node);
            }
        }
        var i :usize = 0;
        while (i<blinkCount): (i+=1) {
            try root.blink(&arragementList);
        }
        total = @intCast(arragementList.len());
    }
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The number of stones will you have after blinking 25 times is {d}\n", .{total});
}