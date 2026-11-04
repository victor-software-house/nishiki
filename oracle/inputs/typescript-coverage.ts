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
