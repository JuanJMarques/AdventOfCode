const std = @import("std");
const root = @import("root.zig");
const Direction = root.Direction;


pub fn main() !void {
    const input = root.readFile("input.txt") catch |err| {std.log.err("{!}", .{err});return err;};
    const result = try root.divideLines(input);
    defer result.deinit();
    var total: u64 = 0;
    const allocator = std.heap.page_allocator;
    var parsingMap = true;
    var boxes = try allocator.alloc([]u8, 0);
    defer {
        for (boxes) |row| {
            allocator.free(row);
        }
        allocator.free(boxes);
    }
    var movements = try allocator.alloc(Direction, 0);
    var robot = root.Robot{.x = 0, .y = 0};
    for (result.items,0..) |line, i| {
        const cleanLine = std.mem.trim(u8, line, "\n\r");
        if (parsingMap){
            if (cleanLine.len == 0){
                parsingMap = false;
            } else {
                boxes = try allocator.realloc(boxes, boxes.len + 1);
                boxes[i] = try allocator.alloc(u8, line.len);
                for (cleanLine, 0..) |c, j| {
                    if  ('@' == c) {
                        boxes[i][j] = '.';
                        robot = root.Robot{.x = j, .y = i};
                    } else {
                        boxes[i][j] = c;
                    }
                }
            }
        }else {
            const oldLength = movements.len;
            movements = try allocator.realloc(movements, movements.len + cleanLine.len);
            for (cleanLine, 0..) |c, j| {
                movements [oldLength+j] = switch (c) {
                    '<' => Direction{.x = -1, .y = 0},
                    '^' => Direction{.x = 0, .y = -1},
                    '>' => Direction{.x = 1, .y = 0},
                    'v' => Direction{.x = 0, .y = 1},
                    else => Direction{.x = 0, .y = 0},
                };
            }
        }
    }
    for (movements) |movement| {
        root.moveRobot(&robot, boxes, movement);
    }

    for (boxes, 0..) |row, i| {
        for (row, 0..) |c, j| {
            if (c == 'O'){
                total += @intCast((100*i + j));
            }
        }
    }
    const stdout = std.io.getStdOut().writer();
    try stdout.print("The sum of all boxes' GPS coordinates is {d}\n", .{total});
}
