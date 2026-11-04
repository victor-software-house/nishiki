#!/usr/bin/env lua
-- Written for nishiki's oracle coverage: the rules of Shiki's Lua grammar
-- that the other inputs do not reach.

---@class Point: Base, Other
---@field x number
---@field private y integer The y coordinate
---@field [string] boolean
---@field protected z? fun(a: integer, b?: string): boolean
---@enum Colour
---@enum (key) Keys
---@type table<string, integer>|nil
---@alias Mode "read" | "write"
---@alias Handler fun(err: string?): nil
---| '"r"' # read
---|> '"w"' # write, the default
---|+ '"a"' # append
---@operator add(Point): Point
---@operator unm: Point
---@cast value string
---@cast value +nil
---@cast value -?
---@param a integer
---@param ... any
---@return boolean ok
---@return string|nil err # why it failed
---@generic T: table, K
---@vararg string
---@overload fun(a: integer): boolean
---@deprecated
---@meta
---@private
---@protected
---@package
---@version >5.2, JIT
---@see Point.x
---@see other#symbol
---@diagnostic disable-next-line: undefined-global, unused-local
---@diagnostic enable
---@module 'mymodule'
---@async
---@nodiscard
---@type `Literal`
local p = setmetatable({}, Point)

--[[@@@
  a documentation block
]]
--[==[@@@ another ]==]
/* a C-style comment */
local s = `backtick ${s}`
local unfinished = `open
local ffi = require("ffi")
ffi.cdef[[
]]
local bad = "\q \k"

local hex = 0xFF
local n = 42
local generic = <T>
local fn = function() end
async(fn)
local logic = a || b && !c
local t = { fn = function(x) return x end, after = 1 }
local function named(a, b: integer) end
---@field [] boolean
---@type <T>
local hexfloat = 0x.8p+1
local tight = a||b&&c
local bang = a!b
