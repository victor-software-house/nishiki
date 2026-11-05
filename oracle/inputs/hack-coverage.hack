<?hh // strict
// Written for nishiki's oracle: it reaches the hack grammar's rules that the
// other Hack inputs miss. Not every line is valid Hack.

namespace Cover\Me;

require_once 'a.php';
include __DIR__ . '/b.php';

module foo.bar;
new module foo.baz {}
module type Alias = int;
newtype Opaque = string;
type Pairs<+Tk as arraykey, -Tv super num> = dict<Tk, Tv>;

public enum class Kinds: IKind extends Base {
  IKind A = new Kind();
}

enum Colour: string as string {
  Red = 'red';
}

public enum Pub: int {}

internal trait Shows {
  require extends Base;
}

public interface Shape extends Drawable, \Cover\Sized {
  const int SIDES = 4;
}

abstract class Square implements \Cover\Me\Drawable, Sized, IFoo {
  const string NAME = 'square', OTHER = "x";
  public static function __construct(): void {}
  final protected async function draw<T>(
    T $t,
    int $n = (1 + 2),
    string $s = Foo::BAR,
  ): void where T as Shape {
    $f = function($x) use (&$y, $z) {
      return $x;
    };
    $g = function() use ($w) {};
    $g2 = public function() {};
    $i1 = $a as $b->c;
    $i2 = $a as $b[0];
    $i3 = $a is $b[$c];
    $i4 = $a upcast $b[d];
    $i5 = $a as ${b};
    $i6 = $a as (int, int);
    $i7 = new $b->c();
    try {
      $o = new parent();
      $p = new static();
      $q = new self();
    } catch (FooException | BarException $e) {
    }
    $$h('a');
    $x->{$name};
    $x->${'b'};
    $x->$prop;
    $x->$$prop;
    ${'dynamic'} = 1;
    ${dynamic} = 2;
    $a = $x[0] . $x[$i] . $x[key] . $x->y;
    $b = $_GET . $_POST . $_COOKIE . $_FILES . $_REQUEST . $argv . $argc;
    $c = $GLOBALS . $_ENV . $_SERVER . $_SESSION;
    $d = array();
    $e = array(1, 2);
    $v = Vector {1, 2};
    $m = ImmMap {'a' => 1};
    $r = $x |> f($$);
    $s2 = $x is int ? $x as int : $x upcast num;
    $t = $a << 1 >> 2 & 3 ^ 4 | 5 ~ 6;
    $u = shape('a' => 1);
    $w = "\x4 \x4a {$x->y} $a->b $a[0] $a[b]";
    $re = re"/a\$\{1,2}[^a]\][*+^$]/i";
    $re2 = re'/\\{1}\\\'[\]\'][$*+^]/x';
    $re3 = re"/\\.\[x]$/";
    $re4 = re'/\\.\.$/';
    $re5 = re"/a{1,2}b{3}/";
    $re6 = re'/a{1,2}b{3}/';
  }
}

function shapes(shape('a' => int, ...) $s, shape(?'b' => string,) $t): void {}

  // An indented comment.

/**
 * A doc comment.
 * @access private
 * @access something else
 * @xlink http://example.com
 * See {@link http://example.com} too.
plain line in a doc comment
 */

$sql = "SELECT a # comment \" here
FROM t -- dash \" comment
WHERE b = \"c\" AND d = `e` AND f = 'g' AND h = \` AND i = `j \` k`";
$sql2 = 'SELECT a # hash \' comment
FROM t -- dash \' comment
WHERE b = \'c\' AND d = `e` AND f = "g" AND h = \"';

$doc = <<<EOT
text $x
EOT;
$quoted = <<<"QUOTED"
text
QUOTED;
$now = <<<'NOW'
text
NOW
;

$xhp = <div>&amp; &#123; &#x1F; &bad;stuff; <!-- a -- b --> <a b='c' d="e"></a></div>;
$sql3 = 'SELECT a FROM `t';
$sql4 = 'SELECT "a';

$xhp3 = <p>&b-c; x</p>;

// One word of each list of built-in functions.
shuffle();
apc_add();
array();
__halt_compiler();
bcadd();
bzclose();
cal_days_in_month();
__autoload();
com_addref();
ctype_alnum();
curl_close();
checkdate();
dba_close();
dbx_close();
chdir();
dotnet_load();
eio_busy();
enchant_broker_describe();
ereg();
debug_backtrace();
escapeshellarg();
exif_imagetype();
basename();
finfo_buffer();
filter_has_var();
call_user_func();
bind_textdomain_codeset();
gmp_abs();
hash();
http_build_cookie();
iconv();
iis_add_server();
gd_info();
assert();
ibase_add_user();
collator_asort();
json_decode();
ldap_8859_to_t61();
libxml_clear_errors();
ezmlm_hash();
abs();
mb_check_encoding();
mcrypt_cbc();
memcache_debug();
mhash();
bson_decode();
mysql_affected_rows();
mysqli_autocommit();
mysqlnd_memcache_get_config();
mysqlnd_ms_get_last_gtid();
mysqlnd_qc_clear_cache();
mysqlnd_uh_convert_to_mysqlnd();
checkdnsrr();
nsapi_request_headers();
aggregate();
oci_bind_array_by_name();
openssl_cipher_iv_length();
flush();
password_get_info();
pcntl_alarm();
pg_affected_rows();
apache_child_terminate();
dom_import_simplexml();
ftp_alloc();
imap_8bit();
mssql_bind();
odbc_autocommit();
preg_filter();
class_implements();
zip_close();
posix_access();
setproctitle();
pspell_add_to_personal();
readline();
recode();
rrd_create();
ftok();
session_cache_expire();
shmop_close();
simplexml_import_dom();
snmp2_get();
is_soap_fault();
socket_accept();
sqlite_array_query();
sqlsrv_begin_transaction();
stats_absolute_deviation();
set_socket_blocking();
addcslashes();
sybase_affected_rows();
is_tainted();
ob_tidyhandler();
token_get_all();
trader_acos();
base64_decode();
boolval();
wddx_add_vars();
xhprof_disable();
utf8_decode();
xmlrpc_decode();
xmlwriter_end_attribute();
xslt_backend_info();
gzclose();

// One word of each list of built-in constants, then each with a leading
// backslash.
DEFAULT_INCLUDE_PATH;
ABDAY_1;
CURLAUTH_ANY;
T_ABSTRACT;
\DEFAULT_INCLUDE_PATH;
\ABDAY_1;
\CURLAUTH_ANY;
\T_ABSTRACT;

// A closing tag with a space after `</` leaves XHP open, so it comes last.
$xhp2 = <x:frag></ x:frag>;
