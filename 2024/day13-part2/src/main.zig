const std = @import("std");
const root = @import("root.zig");
const Button = root.Button;

pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    var parsingA = true;
    var parsingB = false;
    var aButton = Button{.x = 0, .y = 0};
    var bButton= Button{.x = 0, .y = 0};
    var price = Button{.x = 0, .y = 0};
    for (result.items) |line| {
        const cleanLine = std.mem.trim(u8, line, "\n\r");
        if (cleanLine.len == 0) {
            parsingA = true;
            parsingB = false;
            total += @intCast(try root.minimizeTokens(aButton, bButton, price));
        }else {
            if (parsingA or parsingB){
                if (parsingA){
                    aButton = try root.parseButton(cleanLine, '+');
                    parsingA = false;
                    parsingB = true;
                } else {
                    bButton = try root.parseButton(cleanLine, '+');
                    parsingB = false;
                }
            } else {
                price = try root.parseButton(cleanLine, '=');
                price.x += 10000000000000;
                price.y += 10000000000000;
            }
        }
    }
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The fewest tokens you would have to spend to win all possible prizes is {d}\n", .{total});
}