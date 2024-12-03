const std = @import("std");
const root  = @import("root.zig");

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var parsedLines = std.ArrayList(root.point).init(std.heap.page_allocator);
    defer parsedLines.deinit();
    for (result.items) |line| {
        const parsed = root.parseLine(line) catch |err| {std.log.err("{!} -> {s}", .{err, line});return err;};
        try parsedLines.append(parsed);
    }

    var converted = try root.transformLists(parsedLines);
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
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The total distance between your lists is: {d}", .{total});
}