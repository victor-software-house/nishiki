<?php
// Every construct of the php grammar, for the coverage report.

namespace Acme\Shop ! {
    class Cart {}
}
namespace Acme\Util;
namespace\helper();

use function Acme\Util\format;
use const Acme\Util\VERSION;
use Acme\A, Acme\B as Bee;

trait Greets {
    use Hello, World {
        Hello::say insteadof World;
        World::say as protected shout;
        World::a as final b;
        World::c as static d;
        ho as final;
        hum as static;
        hi as public;
        bye as farewell;
    }
}

interface Shape extends Drawable, Sized {
    const int|string KIND = 'shape';
    public function area(): float;
}

enum Suit: string {
    case Hearts = 'H';
    case Spades = 'S';
}

#[\Acme\Attr\Route('/'), Other(1), Override, Plain]
final readonly class Point extends \Acme\Base\Model implements Shape {
    public private(set) ?int $count = 0;
    protected static readonly array $cache;
    final const X = 1;

    final public function __construct(private(set) int $a = 1, readonly string $b, private &$c, public int $d): void {
    }

    public static function &instance(): static {
        return new static();
    }

    abstract public function f();

    public function stop(): never {
        throw new \RuntimeException('no');
    }

    public function params(int &...$rest, ?Foo &$ref = null, &$plain, &$def = [], (A&B)|null $dnf, self|parent $p): void {
    }

    public function defaults($pair = 1 => 2, $a = [1 => 2], $b = array(3), $c = Foo::BAR, $d = &$e, $f = 1 = 2) {
    }
}

$anon = new #[Attr] readonly class (1, 2) {
};
$copy = clone $obj;
$self = new self;
$ref =& $value;
f(&$x);
@$quiet;
$i++;
--$j;
$bits = $a << 2 ^ ~$b | $c >> 1 & $d;
global $config;
var $legacy;
const;
goto done;
    done:
again:

function gen() {
    yield from inner();
}

try {
    risky();
} catch (\Acme\Error | LogicException $e) {
    report($e);
} finally {
    cleanup();
}

$closure = function &() use (&$total, $count): ?int {
    return $total;
};
$arrow = fn &($x): int => $x * 2;
$plain = fn($y) => $y;

$m = match ($value) {
    1, 2 => 'low',
    default => 'high',
};

if ($ready) switch ($mode) {
    case 1:
        break;
}
switch ($x):
endswitch;

$t = $flag ?
    yes : no;

$arr = array();
$arr2 = array(1, 2);
$cast = (real) $n;

\Acme\Util\helper();
Acme\Support\call();
$class = \ArrayIterator::class;
echo E_ALL, \INF, DNS_A, T_SR, \E_ALL, \DNS_A, \T_SR;

$hex = 0x1F_FF;
$bin = 0b1010_1;
$oct = 0o17_7;
$old = 017;
$float = 1_000.5e-3;
$half = .5;
$dot = 1.;
$exp = 1e3;

$escapes = "\101 \x41 \u{1F600} \n \$ \"quoted\"";
$members = "$a->b $a?->b $a[1] $a[$i] $a[key] ${name}";
$dynamic = ${'a' . 'b'};
$globals = $_GET['q'] . $GLOBALS['x'] . $_SERVER['y'] . $argv[0];
$shell = `ls \` -l`;

preg_match("/a\.b{1,2}[^x]$/i", $s);
preg_match('/a{1,2}/', $s);

$q1 = "SELECT * FROM t # trailing comment";
$q2 = "SELECT 1 -- a comment";
$q3 = "SELECT \"x\", 'y', `col` FROM t";
$q4 = "SELECT `a` FROM t";
$q5 = 'SELECT * FROM t # comment';
$q6 = 'SELECT 1 -- comment';
$q7 = 'SELECT \'x\', N\'y\', `col`, "z" FROM t';
$q8 = "SELECT 'abc";
$q9 = "SELECT `abc";
$q10 = 'SELECT N';
$q11 = 'SELECT `abc';
$q12 = 'SELECT "abc';

/**
 * A doc comment.
 not starting with a star
 * @access public
 * @access other
 * @xlink http://example.com
 * @param A|B $x
 * @param A| B $w
 * @param (A| B)[] $v
 * @param ?int $y
 * @param (A|B)[] $z
 * @return self {@link Foo}
 */

/* a block comment */
# a hash comment
    // an indented comment

$h1 = <<<HTML   
<p>{$a}</p>
HTML;
$h2 = <<<XML   
<a/>
XML;
$h3 = <<<SQL   
SELECT 1
SQL;
$h4 = <<<JS   
let a = 1;
JS;
$h5 = <<<JSON
{"a": 1}
JSON;
$h6 = <<<CSS   
a { color: red; }
CSS;
$h7 = <<<REGEX   
a\.b{1,2}[\]x] # a comment
REGEX;
$h8 = <<<BLADE
<div>{{ $a }}</div>
BLADE;
$h9 = <<<EOT
plain $text
EOT;

$n1 = <<<'HTML'  
<p>$a</p>
HTML;
$n2 = <<<'XML'
<a/>
XML;
$n3 = <<<'SQL'
SELECT 1
SQL;
$n4 = <<<'JS'
let a = 1;
JS;
$n5 = <<<'JSON'  
{"a": 1}
JSON;
$n6 = <<<'CSS'
a { color: red; }
CSS;
$n7 = <<<'REGEX'
a\.b{1,2}[\]x]$ # a comment
REGEX;
$n8 = <<<'BLADE'
<div>{{ $a }}</div>
BLADE;
$n9 = <<<'EOT'
plain $text
EOT;

// One function of each support list.
apc_cas();
end();
constant();
bcadd();
blenc_encrypt();
bzopen();
cal_info();
is_a();
variant_or();
isset();
echo();
ctype_space();
curl_exec();
time();
dba_sync();
dbx_sort();
dir();
eio_nop();
enchant_dict_check();
ereg();
error_log();
exec();
exif_tagname();
fann_run();
stat();
finfo_open();
filter_id();
fpm_get_status();
func_get_arg();
gettext();
gmp_or();
hash();
http_get();
iconv();
iis_add_server();
gd_info();
dl();
ibase_close();
idn_to_utf8();
json_decode();
ldap_tls();
libxml_get_errors();
mail();
pi();
mb_ord();
mcrypt_cfb();
memcache_debug();
mhash();
log_reply();
mysql_stat();
mysqli_stat();
mysqlnd_memcache_set();
mysqlnd_ms_xa_gc();
mysqlnd_qc_clear_cache();
mysqlnd_uh_convert_to_mysqlnd();
syslog();
nsapi_virtual();
ociparse();
opcache_reset();
openssl_sign();
flush();
password_hash();
pcntl_exec();
pg_();
virtual();
dom_import_simplexml();
ftp_put();
imap_gc();
mssql_init();
odbc_do();
preg_grep();
class_uses();
zip_open();
posix_kill();
setproctitle();
pspell_new();
readline();
recode();
rrd_tune();
ftok();
session_id();
shmop_size();
simplexml_load_file();
snmpget();
is_soap_fault();
socket_bind();
sqlite_key();
sqlsrv_close();
stats_skew();
stream_isatty();
md5();
sybase_close();
taint();
tidy_getopt();
token_name();
trader_t3();
uopz_copy();
urldecode();
intval();
wddx_add_vars();
xhprof_enable();
xml_parse();
xmlrpc_decode();
xmlwriter_();
gzeof();
