const std = @import("std");
const testing = std.testing;

pub fn divideLines(input: []const u8) !std.ArrayList([]const u8) {
    const allocator = std.heap.page_allocator;
    var lineList = std.ArrayList([]const u8).init(allocator);
    var lines = std.mem.splitSequence(u8, input, "\n");
    while (lines.next()) |line| {
        try lineList.append(std.mem.trim(u8, line, "\r\n"));
    }
    return lineList;
}

pub fn readFile(path: []const u8) ![]const u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return try file.readToEndAlloc(std.heap.page_allocator, @as(usize, std.math.maxInt(usize)));
}

pub const Robot = struct {
    x: usize,
    y: usize,
};

pub const Direction = struct {
    x: i32,
    y: i32,
};

pub fn moveRobot(robot: *Robot, boxes: [][]u8, movement: Direction) void {
    var newX :i32 = @intCast(robot.x);
    newX += movement.x;
    var newY :i32 = @intCast(robot.y);
    newY += movement.y;
    if (newX <= 0 or newY <= 0 or newY >= boxes.len - 1 or newX >= boxes[0].len - 1) {
        return;
    }

    var queue = std.DoublyLinkedList(u8){};

    var node = std.DoublyLinkedList(u8).Node{.data = '.'};
    queue.append(&node);
    var ux: usize = @intCast(newX);
    var uy: usize = @intCast(newY);
    var data = boxes[uy][ux];
    while (data != '#' and data != '.') {
        node = std.DoublyLinkedList(u8).Node{.data = data};
        queue.append(&node);
        newX += movement.x;
        newY += movement.y;
        if (newX <= 0 or newY <= 0 or newY >= boxes.len - 1 or newX >= boxes[0].len - 1) {
            return;
        }
        ux = @intCast(newX);
        uy = @intCast(newY);
        data = boxes[uy][ux];
    }
    if (data == '#') {
        return;
    }
    if (robot.x == newX) {
        while (newY != robot.y): (newY -= movement.y) {
            const newValue = queue.pop() orelse unreachable;
            boxes[@intCast(newY)][ux] = newValue.data;
        }
    } else {
        while (newX != robot.x): (newX -= movement.x) {
            const newValue = queue.pop() orelse unreachable;
            boxes[uy][@intCast(newX)] = newValue.data;
        }
    }
    newX = @intCast(robot.x);
    newX += movement.x;
    newY = @intCast(robot.y);
    newY += movement.y;
    robot.x = @intCast(newX);
    robot.y = @intCast(newY);
    boxes[robot.y][robot.x] = '.';
}

test "test case" {
    const input =
        \\########
        \\#..O.O.#
        \\##@.O..#
        \\#...O..#
        \\#.#.O..#
        \\#...O..#
        \\#......#
        \\########
        \\
        \\<^^>>>vv<v>>v<<
    ;

    const result = try divideLines(input);
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
    var robot = Robot{.x = 0, .y = 0};
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
                        robot = Robot{.x = j, .y = i};
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

    const out = std.io.getStdOut().writer();
    for (movements) |movement| {
        moveRobot(&robot, boxes, movement);
        for (boxes) |value| {
            try out.print("{s}\n", .{value});
        }
        try out.print("\n", .{});
    }

    for (boxes, 0..) |row, i| {
        for (row, 0..) |c, j| {
            if (c == 'O'){
                total += @intCast((100*i + j));
            }
        }
    }

    try testing.expectEqual(2028, total);
}

test "test case 2" {
    const input =
        \\##########
        \\#..O..O.O#
        \\#......O.#
        \\#.OO..O.O#
        \\#..O@..O.#
        \\#O#..O...#
        \\#O..O..O.#
        \\#.OO.O.OO#
        \\#....O...#
        \\##########
        \\
        \\<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        \\vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        \\><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        \\<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        \\^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        \\^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        \\>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        \\<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        \\^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        \\v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    ;

    const result = try divideLines(input);
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
    var robot = Robot{.x = 0, .y = 0};
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
                        robot = Robot{.x = j, .y = i};
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

    const out = std.io.getStdOut().writer();
    for (movements) |movement| {
        moveRobot(&robot, boxes, movement);
        for (boxes) |value| {
            try out.print("{s}\n", .{value});
        }
        try out.print("\n", .{});
    }

    for (boxes, 0..) |row, i| {
        for (row, 0..) |c, j| {
            if (c == 'O'){
                total += @intCast((100*i + j));
            }
        }
    }

    try testing.expectEqual(10092, total);
}