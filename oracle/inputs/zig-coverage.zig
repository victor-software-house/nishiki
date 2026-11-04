//! Written for nishiki: Zig that reaches the grammar's rarer rules.
const c = @cImport(@cInclude("stdio.h"));

fn @"quoted name"(x: c_int, y: c_ulonglong) c_long {
    inline for (.{ 1, 2 }) |v| {
        _ = v;
    }
    const frame = async run();
    suspend {}
    resume frame;
    nosuspend run();
    const bad = "\q \x41 \u{1F600}";
    const odd = 1x + 0x_1F + 2e9z;
    return x == y;
}
