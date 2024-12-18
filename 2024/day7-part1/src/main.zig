const std = @import("std");
const root  = @import("root.zig");

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const allocator = std.heap.page_allocator;
    for (result.items) |line| {
        const cleanline = std.mem.trim(u8, line, "\r\n");
        var parts = std.mem.splitSequence(u8, cleanline, ":");
        const totalStr = parts.next().?;
        const objective = try std.fmt.parseInt(u64, totalStr, 10);
        const operands = parts.next().?;
        parts = std.mem.splitSequence(u8, operands, " ");
        var operandsList = std.ArrayList(u64).init(allocator);
        defer operandsList.deinit();
        while (parts.next()) |operandStr| {
            if(operandStr.len > 0){
                const operand = try std.fmt.parseInt(u64, operandStr, 10);
                try operandsList.append(operand);
            }
        }
        if(root.findOperations(objective, operandsList.items, '+', 0)){
            total += objective;
        }
    }
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The total calibration result is: {d}\n", .{total});
}