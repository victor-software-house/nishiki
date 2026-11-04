<?php
// Strings and heredocs that PHP's grammar hands to other languages.

$byId = "SELECT id, name FROM users WHERE id = $id";
$count = 'SELECT COUNT(*) FROM orders WHERE total > 10';
$update = "UPDATE users SET name = '{$name}' WHERE id = 1";
$plain = "not a query, with $value inside";

$page = <<<HTML
<div class="card">
  <a href="/users/{$id}">{$name}</a>
</div>
HTML;

$static = <<<'HTML'
<p>No $interpolation here.</p>
HTML;

$feed = <<<XML
<?xml version="1.0"?>
<feed><entry id="1">{$title}</entry></feed>
XML;

$query = <<<SQL
SELECT u.id, COUNT(o.id) AS orders
FROM users u
LEFT JOIN orders o ON o.user_id = u.id
GROUP BY u.id
SQL;

$script = <<<JS
const total = items.reduce((sum, item) => sum + item.price, 0);
console.log(`total: ${total}`);
JS;

$config = <<<'JSON'
{"name": "app", "debug": false, "ports": [80, 443]}
JSON;

$style = <<<CSS
.card { color: #333; margin: 0 auto; }
CSS;

$pattern = <<<REGEXP
/^[a-z]+\d*$/
REGEXP;

echo strlen($page) + strlen($query);
