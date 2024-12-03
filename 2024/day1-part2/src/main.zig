const std = @import("std");
const root  = @import("root.zig");

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var parsedLines = std.ArrayList(root.point).init(std.heap.page_allocator);
    defer parsedLines.deinit();
    var keys = try std.ArrayList(i32).initCapacity(std.heap.page_allocator, parsedLines.capacity);
    defer keys.deinit();
    var values = std.AutoHashMap(i32, i32).init(std.heap.page_allocator);
    defer values.deinit();
    for (result.items) |line| {
        const parsed = try root.parseLine(line);
        try keys.append(parsed.a);
        try values.put(parsed.b, if (!values.contains(parsed.b)) 1 else values.get(parsed.b).? + 1);
    }

    var total: i32 = 0;
    for(keys.items) |key| {
        if (values.contains(key)) {
            total += key * values.get(key).?;
        }
    }
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The similarity score between your lists is: {d}", .{total});
}