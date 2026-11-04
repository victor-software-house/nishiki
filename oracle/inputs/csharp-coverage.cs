// C# that nishiki's other oracle inputs leave out, written to exercise the
// rest of Shiki's `csharp` grammar.
#:package Newtonsoft.Json@13.0.3
#:sdk Microsoft.NET.Sdk.Web@9.0.0
#:property LangVersion=preview
#:project ../Shared/Shared.csproj
#:include Helpers/*.cs
#:exclude Helpers/Old.cs
#:ref Other.dll
#:custom anything here
#r "System.Net.Http"
#load "helpers.csx"
#define TRACE_ON /* traced */
#undef TRACE_OFF
#if (TRACE_ON && !TRACE_OFF) || DEBUG == true
#warning Tracing is on
#elif RELEASE != false
#error Not a release
#endif
#line 200 "Generated.cs"
#line default
#line hidden
#pragma warning disable CS0168, 0219
#pragma warning restore 168, 219
#pragma checksum "file.cs" "{406ea660-64cf-4c82-b6f0-42d48172a799}" "ab007f1d23d9"
#region Setup /* a comment */
#endregion
extern alias GridV1;
global using System;
global using static System.Math;
using static unsafe System.Buffer;
using global::System.Text;
using Alias = System.Collections.Generic.List<int>;
using unsafe IntPtrAlias = int*;

namespace Coverage;

public class Base(int n, string s);

/// <summary>
/// A <c kind="x" xml:lang="en">class</c> <x:tag/> &amp; its <see cref="Base"/> &#169; &#xA9; & more.
/// <![CDATA[ raw <text> ]]>
/// <!-- a comment -->
/// </summary>
/** A block documentation comment. */
/**
 * Spread over lines.
 */
public sealed class Shapes<in TIn, out TOut>(int n) : Coverage.Base<int>(n, "x") where TIn : class
{
    private readonly int count = 0b_1010_1010;
    private const double Small = 1_000.000_5e-3_0d, Tiny = .5e+10f, Money = 1.5m;
    private const double Exp = 1e+1_0, Neg = 2E-5, Thousand = 1_000;
    private const ulong Big = 0xFF_FFul, Bin = 0b1010UL, Long = 3L, Hex = 0x1Fu, Wide = 10UL;
    private int bad = 12ab;
    // An indented line comment.
    /// An indented documentation comment.
    ~Shapes() { }
    public event EventHandler Changed = delegate { }, Removed;
    public int this[int index, string key = "k"] => index;
    public static implicit operator int(Shapes<TIn, TOut> s) => s.count;
    public static explicit operator Shapes<TIn, TOut>(int n) => new(n);
    public int Value { get; private set; }
    public int Fixed { readonly get => 1; }
    int IList<int>.this[int i] { get => i; set { } }
    public void Defaults(int x = 5, ref readonly int y) { }

    public IEnumerable<int> Numbers()
    {
        do { count++; } while (count < 10);
        yield break;
    }

    public string Describe(object shape, int[] values)
    {
        const int Limit = 3, Other = 4;
        (int a, int b) = (1, 2);
        var (x, y) = (a, b);
        (a, b) = (b, a);
        var verbatim = $@"{a} ""quoted"" {b}";
        var verbatim2 = @$"{a}";
        var path = @"C:\""dir""";
        var raw4 = $""""{a}"""";
        var raw4d = $$""""{{a}}"""";
        var raw5 = $"""""{a}""""";
        var raw3 = $$$"""{{{a}}}""";
        var plain5 = """""five quotes""""";
        var chars = values[^1..];
        var maybe = shape?.ToString()?.Length;
        var items = values?[0];
        var nested = this?.Names[0];
        var generic = Shapes<int, int>.Empty.Count;
        int.TryParse("1", out var parsed);
        ref readonly var first = ref values[0];
        Func<int[], int> pick = arr => ref arr[0];
        Func<int, int> twice = (ref int v) => v * 2;
        T Id<T>(T t) => t;
        var unterminated = "no end
        var unterminatedChar = 'x
        var unterminatedInterpolation = $"no end
        var escaped = $"{{{a}}}";
        var rawEscaped = $"""{{{a}}}""";
        if (shape is < 0 or > 10) { }
        if (values is [1, .., var tail]) { }
        if (shape is global::Coverage.Base b2) { }
        if (shape is int? n2) { }
        var element = this[0, "k"];
        var call = Describe<int>(shape, values);
        var member = shape.ToString().Length;
        switch (shape)
        {
            case int i when i > 0:
                goto default;
            case (Base)Shapes:
                break;
            default:
                break;
        }
        var answer = shape switch
        {
            < 0 => "negative",
            >= 10 and <= 20 => "teens",
            (int)Limit => "limit",
            (int)(Limit) => "nested",
            (int)global::Coverage.Shapes => "alias",
            (int)System.Int32.MaxValue => "max",
            System.String s => s,
            int[] { Length: > 0 } list when list is [1, .., var last] => last.ToString(),
            Base { } b? => "base",
            _ => "other",
        };
        Func<int, int> add = (int v) => v + 1;
        foreach (var (key, item) in new Dictionary<int, string>()) { }
        foreach (ref var cell in values.AsSpan()) { }
        var query = from int v in values
                    join int w in values on v equals w into joined
                    select v;
        {(int e, string f) = (1, "x");}
        unsafe
        {
            var p = &e;
            p->X = 1;
            p->M();
            p->arr[0] = 1;
        }
        var empty = Coverage.Shapes<int, int>.Empty;
        var withDefaults = (int x = 5, int y = 6) => x + y;
        var suffixed = 1e+1_0UL + 2E-5L + 1_000UL;
        return answer;
    }
}

public enum Colors : byte
{
    Red = 1,
    Green = 2,
}
