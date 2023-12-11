const std = @import("std");
const testing = std.testing;
const PUZZLE_INPUT = @embedFile("../input.txt");

fn process_line(input: []const u8) u32 {
    var first: ?u32 = null;
    var last: u32 = 0;
    for (input, 0..) |_, i| {
        if (to_value(input[i..])) |val| {
            if (first == null) {
                first = val;
            }
            last = val;
        }
    }
    return (first orelse 0) * 10 + last;
}

const Pattern = struct { pattern: []const u8, value: u32 };

const PATTERNS = [9]Pattern{ .{ .pattern = "1", .value = 1 }, .{ .pattern = "2", .value = 2 }, .{ .pattern = "3", .value = 3 }, .{ .pattern = "4", .value = 4 }, .{ .pattern = "5", .value = 5 }, .{ .pattern = "6", .value = 6 }, .{ .pattern = "7", .value = 7 }, .{ .pattern = "8", .value = 8 }, .{ .pattern = "9", .value = 9 } };

fn to_value(input: []const u8) ?u32 {
    for (PATTERNS) |pattern| {
        if (std.mem.startsWith(u8, input, pattern.pattern)) {
            return pattern.value;
        }
    }
    return null;
}

test "process line" {
    try testing.expect(process_line("1akefkj2") == 13);
    try testing.expect(process_line("a2kefkj2") == 22);
}
