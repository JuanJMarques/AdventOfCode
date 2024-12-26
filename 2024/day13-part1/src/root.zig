const std = @import("std");
const testing = std.testing;
pub const Button  = struct {
    x: i32,
    y: i32,
};

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

pub fn parseButton(line: []const u8, ch: u8) !Button {
    var x: i32 = 0;
    var y: i32 = 0;
    const values = line[(std.mem.indexOfScalar(u8, line, ':') orelse 0) + 1 ..];
    var it2 = std.mem.split(u8, values, ",");
    if (it2.next()) |xStr| {
        const cleanVal = xStr[(std.mem.indexOfScalar(u8, xStr, ch) orelse 0) + 1 ..];
        x = try std.fmt.parseInt(i32, cleanVal, 10);
    }
    if (it2.next()) |yStr| {
        const cleanVal = yStr[(std.mem.indexOfScalar(u8, yStr, ch) orelse 0) + 1 ..];
        y = try std.fmt.parseInt(i32, cleanVal, 10);
    }
    return Button{.x = x, .y = y};
}

const Candidate = struct {
    x: i32,
    y: i32,
    cost: i32,
};


fn lessThan(context: Button, a: Candidate, b: Candidate) std.math.Order {
    return std.math.order((context.x - a.x) + (context.y - a.y) + a.cost, (context.y - b.y) + (context.y - b.y) + b.cost);
}

pub fn minimizeTokens(aButton: Button, bButton: Button, prize: Button) !i32 {
    const aTokens: i32 = 3;
    const bTokens: i32 = 1;
    const allocator = std.heap.page_allocator;
    var minCost: i32 = std.math.maxInt(i32);

    var list = std.PriorityQueue(Candidate, Button, lessThan).init(allocator, prize);
    defer list.deinit();
    var history = std.AutoHashMap(Candidate, void).init(allocator);
    defer history.deinit();
    try list.add( Candidate{
        .x = 0,
        .y = 0,
        .cost = 0,
    });
    while (list.removeOrNull()) |candidate| {
        try history.put(candidate, {});
        if(candidate.x <= prize.x and candidate.y <= prize.y) {
            if (candidate.x == prize.x and candidate.y  == prize.y) {
                minCost = @min(minCost, candidate.cost);
            } else {
                if (candidate.cost < minCost){
                    var nextCandidate = Candidate{
                        .x = candidate.x + aButton.x,
                        .y = candidate.y + aButton.y,
                        .cost = candidate.cost + aTokens,
                    };
                    if (!history.contains(nextCandidate)){
                        try list.add(nextCandidate);
                    }
                    nextCandidate = Candidate{
                        .x = candidate.x + bButton.x,
                        .y = candidate.y + bButton.y,
                        .cost = candidate.cost + bTokens,
                    };
                    if (!history.contains(nextCandidate)){
                        try list.add(nextCandidate);
                    }
                }
            }
        }
    }
    if (minCost == std.math.maxInt(i32)){
        minCost = 0;
    }
    return minCost;
}


test "test case" {
    const input =
        \\Button A: X+94, Y+34
        \\Button B: X+22, Y+67
        \\Prize: X=8400, Y=5400
        \\
        \\Button A: X+26, Y+66
        \\Button B: X+67, Y+21
        \\Prize: X=12748, Y=12176
        \\
        \\Button A: X+17, Y+86
        \\Button B: X+84, Y+37
        \\Prize: X=7870, Y=6450
        \\
        \\Button A: X+69, Y+23
        \\Button B: X+27, Y+71
        \\Prize: X=18641, Y=10279
        \\
    ;

    const result = try divideLines(input);
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
            total += @intCast(try minimizeTokens(aButton, bButton, price));
        }else {
            if (parsingA or parsingB){
                if (parsingA){
                    aButton = try parseButton(cleanLine, '+');
                    parsingA = false;
                    parsingB = true;
                } else {
                    bButton = try parseButton(cleanLine, '+');
                    parsingB = false;
                }
            } else {
                price = try parseButton(cleanLine, '=');
            }
        }
    }

    try testing.expectEqual(480, total);
}