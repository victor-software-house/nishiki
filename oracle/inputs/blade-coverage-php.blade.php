{{-- Blade's own copy of PHP, which @php blocks use, and the rest of the blade grammar. --}}
Text :a="$x" and :data-id="$id" outside a tag.
<x-card :title="$title" />
@php
    $a = new namespace\Foo();
    $b = namespace\Bar::baz();
    $c = namespace\VALUE;
@endphp
{{--
    <?php
    $hidden = 1;
    ?>
--}}
<p>Before <?php echo $open;
?> after</p>
@section('a') @show() @stop ($x) @parent() @endif ( $y )
<div :class="active" :data-id="$id"></div>
@php
    namespace\helper();
    \strlen($this->name);
    abstract class Shape {}
    final class Square {}
    function legacy(array &$a = array(1), ...$rest = [1, [2]]) {}
    trait Uses {
        use T { a as protected b; c as public; }
    }
    $ok = is_int($a) || is_integer($b);
    preg_match('/a\'b\\c\.d/', $s);
    $re = <<<REGEX
^a+b*c$
REGEX;
@endphp
