#!/usr/bin/env node
// TypeScript that nishiki's other oracle inputs leave out, written to
// exercise the rest of Shiki's `typescript` and `javascript` grammars.
// @internal
    // @internal
import type Default, { type Named } from "./module" with { type: "json" };
import data from "./data.json" assert { kind: /* c */ "json" };
export import Alias = Outer.Inner?.Name;
export declare import Lazy = Outer.Inner;
export import fs = require("fs");
import type req = require("req");
export type { Named };
export type = Named;
export default class {}

/**
 * @access private
 * @api public
 * @author Jane Doe <jane@example.com>
 * @borrows trstr as trim
 * @example <caption>Usage</caption>
 * run();
 * @kind function
 * @see https://example.com/docs
 * @see Other.method
 * @default "quoted value"
 * @license 'MIT'
 * @version 1.2.3
 * @tutorial getting-started
 * @variation 2
 * @param {string} [name="x"] described
 * @param {string} [opt]junk described
 * See [label]{@link Other} and {@link https://example.com|Example} and {@link Target|text}.
 */
export declare abstract interface Shape {
    readonly [key: string]?: unknown;
    new (): Shape;
}

export declare const enum Flags {
    ["computed"] = 1,
    'quoted' = 2,
    `template` = 3,
}

type Mapped<T> = { -readonly [K in keyof T]-?: T[K] };
type Added<T> = { +readonly [K in keyof T]+?: T[K] };
type Tuple = [first?: string, ...rest: number[]];
type Fn = (this: Window, ...args: number[]) => void;
type Ctor = new () => Shape;
type AbstractCtor = abstract new () => Shape;
type Name = Outer?.Inner;
type Tagged = `prefix-${string}`;

function assertString(value: unknown): asserts value is string {}
function assertThis(this: Shape): asserts this {}

class Base<T> extends Outer?.Inner<T> implements Shape {
    prop!: string;
    handler! = () => {};
    #secret = 0;
    [key: string]: unknown;
    override public abstract async constructor(public readonly x: number, override protected y?: string) {
        super();
    }
    public async *[Symbol.iterator]() {}
    override public abstract async new() {}
    get value() { return this.#secret; }
    set value(v) { this.#secret = v; }
}

const Anonymous = abstract class {};
const Expression = class Named {};
const generator = function* named() {};
const asyncFn = async function* () {};
const single = async x => x;
const paren = async (x) => x;
const object = {
    get size() { return 1; },
    set size(v) {},
    async *gen() {},
    async *[Symbol.iterator]() {},
    get [computed]() { return 1; },
    method<T>(): void {},
    ...spread,
    "quoted": 1,
    42: 2,
    shorthand,
    key: function () {},
    arrow: () => 1,
    later: async (x) => x,
};

function params({ ...rest }, [...items], ...{ length }) {}
function more(...[first, second]) {}
let definite!: number;
const { a: { ...inner }, ...others } = object;
const [x, ...[y, z]] = list;
const [...{ length: count }] = list;

let bits = 0;
bits &= 1; bits ^= 2; bits <<= 3; bits >>= 4; bits >>>= 5; bits |= 6;
delete object.key;
if (object instanceof Base) {}
const made = new Base<number>();
const target = new.target;
const ratio = value! / 2;
let quotient = value!;
quotient! /= 2;
const divided = value /* why */ / 2;
const regex = /(?<=a)(?<!b)(?<name>c)\k<name>[^a-z\d]{1,3}\p{L}\u{1F600}/giu;
if (/x/gi.test(text)) {}
const numbers = [1.5e+10, 1.e5, .5e-5, 2e3, 3.14, 4., .25, 10n, 1_000n, 0x1F_FFn, 0b1010n, 0o777n];
fn?.();
fn!();
obj?.method = function () {};
obj?.CONSTANT;
obj.CONSTANT_TWO;
const raw = String.raw`line\n${value}`;
const tagged = tag<string>`text`;
process.env.HOME;
module.exports = {};
require.main;
exports.value = 1;
arguments.length;
debugger;
package;

/* @internal */ export const hidden = 1;
export declare import type Shape from "./shape";
declare import fs = require("fs");
import type Alias = Outer.Inner;
import { default as main, type kind as Kind, "quoted-name" as quoted } from "./names";
import { default } from "./plain";
export { main as default, kind as "string-alias", "bare-string" };
export { type main as renamed };
export * from "./everything";

let [first, ...[second, third]] = pairs;
let { alpha, ...{ beta } } = record;
function spreadArray(...[a, b]: number[]) {}
function spreadObject(...{ length }: string) {}
function nestedArray([head, ...[next]]: number[]) {}
function nestedObject({ key, ...{ rest } }: Bag) {}
function withThis(this: () => void, ...callbacks: () => void) {}

const numbers = [1.5e3n, 1.e3n, .5e3n, 1e3n, 1.5n, 1.n_, .5n];
const infinite = Infinity;
const value = POINTS.LIMIT.max;
const max = Number.MAX_SAFE_INTEGER;
const meta = import?.meta;
const loaded = import("./lazy");
module?.exports;

function construct() {
  return new?.target;
}

function taggedTarget() {
  return new?.target`x`;
}

const quotient = total /* per item */ / count;
let ratio = 10;
ratio /* halve */ /= 2;

async function iterate(stream: AsyncIterable<number>) {
  for await (const chunk of stream) {
  }
}

function /* named */ * generate() {}

if (ready) /ab+c/g.test(input);

const patterns = [/[\x41\u0041\101\cA]/, /a(?=b)/, /[\x41-\x5a\cA-\cZa-\]]/];

case 0:
default:
switch

const arrow = async (
  public readonly a: number,
  private readonly cb: () => void,
  public ...callbacks: () => void,
  readonly this: () => void,
  maybe?: () => void,
  readonly ...rest: string[]
) => a;

function returns() {
  return async (
    public this: Window,
    b?: number
  ) => b;
}

const methods = {
  42 /* answer */ : "answer",
  async: async <T>(
    x: T
  ) => x,
  plain: async (
    y: number
  ) => y,
  generic: async <
    T
  >(
    z: T
  ) => z,
  "quoted" (): void {},
  first()
  async *second() {},
  third()
  get fourth() {},
  [a[b[c]]]() {},
  ٤٢: 1,
  "k" as const,
  "s" satisfies Shape,
  "t" as Shape,
};
const checked = { shorthand satisfies Shape };

class Shapes {
  override draw() {}
  abstract area(): number;
  *() {}
  x = flag ? 1 : 2;
  flag ? on : off
}

type Upper<S extends string> = intrinsic;
type Ctor = new <T>(x: T) => T;
type AbstractCtor = abstract new <T>() => T;
type Head<T> = T extends [infer H extends string, ...unknown[]] ? H : never;
type Member = ns?.<T>;
type Spread = { ...rest };
type Fn = (readonly cb: () => void, ...fns: () => void, this: () => void, maybe?: () => void) => void;
type Plain = (readonly a: string, ...b: string[], this: Window, c?: number) => void;
function assertsOnly(): asserts {}
function isOnly(): is {}

import // @internal
  lateName from "./late";
const c0 = 0, // @internal
  c00 = 0;
const c1 = 1,
  c2 = 2;
let l1 = 1,
  l2 = 2;
var v1 = 1,
  v2 = 2;
using u1 = r1,
  u2 = r2;

let handler!: () => void;
let LIMIT!: number;
const tagged = tag<string>`text`;
const spaced = tag <string>`text`;
export abstract class Base extends ns.Parent {}
export declare abstract class Declared {}
@sealed
class Sealed {
  async
  cb? = () => 0;
  accessor count = 0;
  constructor(private readonly onDone: () => void) {}
}
enum Colour {
  Red,
  Green = 2,
}
outer: {
  break outer;
}
let { 0: zero } = list;
const shifted = (a << 2) + (b >> 1) + (c >>> 3);
const bits = (a & b) | (c ^ ~d);
const fits = shape satisfies Shape;
const deep = a?.b.c;
const here = import.meta;
const size = list?.length;
const singleUnterminated = 'no end
type Mapped<T> = { [K in keyof T as Upper<K>]: T[K] };
declare type Declared = 1;
type Lazy = import("./lazy").Value;
type Later = () =>
  void;
type Optional = (a?: string) => void;
let annotated:
  number;
function laterReturn():
  number {}
function defaulted<T = string>() {}
let frozen: readonly string[];
class AsyncStatic { async static one() {} }
class Guards {
  isThing(): this is Thing {}
}
function assertsValue(x): asserts x {}
export declare using resource = acquire();
const groups = [/(?!x)/, /(?:y)/, /[\--z]/, /[\.-9]/];
const unterminated = "no end
