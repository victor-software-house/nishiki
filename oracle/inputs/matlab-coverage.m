% Written for nishiki: MATLAB that reaches the rules the other inputs leave
% out, as tests/oracle/COVERAGE.md counts them.

%% Section title
  %% Indented section
%%
  % indented comment

%{
A block comment.
  %{
  A nested block.
  %}
%}

hold on
format long g % trailing comment
disp hello world
warning off all
axis([0 1 0 1])
!ls -la
  !echo shell

global counter total
persistent cache

x = a' + b';
y = [a ' str'];
z = {x', y.', m(1)', n{2}'};
s = 'it''s a quote';
t = "a "" quote";
u = 'unterminated ' quote';
v = "unterminated " quote";
u2 = 'a lone'quote';
v2 = "a lone"quote";
w = sprintf('%d items, %5.2f%% done\n', n, p);
c = a(end) + b{end};
d = 1e-3 * 2.5i + .5 - 3j;

parfor k = 1:10
    r(k) = k^2;
end

spmd
    q = labindex;
end

spmd (4)
    q = 1;
end

try
    risky();
catch err
    disp(err.message);
end

try, risky(); catch, end

while true
    break
end

if x > 0
    y = 1;
elseif x < 0
    y = -1;
else
    y = 0;
end

if x, y = 1; else y = 2; end

switch s
    case 'a'
        z = 1;
    case {'b', 'c'}
        z = 2;
    otherwise
        z = 3;
end

switch t
    otherwise z = 4;
end

function [a, b] = two_outputs(x, y)
    a = x;
    b = y;
end

function single = one_output(x)
    single = x;
end

function no_outputs(varargin)
    disp(nargin);
end

classdef (Sealed, Abstract = true) Shape < handle & matlab.mixin.Copyable % a shape
    properties (Access = private, Constant = false)
        width (1,1) double {mustBePositive} = 1
        height {mustBeNumeric, mustBeFinite}
        name string = "shape"; label = 'none'
        tags cell {mustBeMember(tags, {'a', "b", 'it''s', "q""q"})}
    end
    properties (SetAccess = protected, GetAccess = public)
        area
    end
    methods (Access = private, Static = true)
        function r = scale(obj, k)
            r = obj.width * k;
        end
    end
    methods
        function obj = Shape(w)
            obj.width = w;
        end
    end
    events (ListenAccess = protected, NotifyAccess = private, Hidden = false)
        Changed
    end
    enumeration
        Small, Large
    end
end

function validate(x, opts)
    arguments
        x (1,:) double {mustBeReal}
        opts.Mode char {mustBeMember(opts.Mode, {'fast', 'slow'})} = 'fast'
        opts.Count
        opts.Name {mustBeMember(opts.Name, {'a'b', "c"d"})}
    end
end
