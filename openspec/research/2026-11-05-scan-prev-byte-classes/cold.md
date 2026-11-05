# Cold start, bench:cold

Medians and p90 in µs, 31 fresh processes each, on one 4x8 build box.

## a251d5ef, swift

| file | new, median | new, p90 | first line, median | first line, p90 | document, median | document, p90 |
|:--|--:|--:|--:|--:|--:|--:|
| swift/small/snapkit-constraint-item.swift | 17.2 | 22.4 | 99.8 | 118.9 | 476.7 | 528.4 |

github-dark from JSON: median 112.1 µs, p90 132.9 µs

## a251d5ef, ruby

| file | new, median | new, p90 | first line, median | first line, p90 | document, median | document, p90 |
|:--|--:|--:|--:|--:|--:|--:|
| ruby/small/faraday-middleware.rb | 17.1 | 18.9 | 91.3 | 107.2 | 391.1 | 420.0 |

github-dark from JSON: median 111.4 µs, p90 123.0 µs

## 1f9d2a72, swift

| file | new, median | new, p90 | first line, median | first line, p90 | document, median | document, p90 |
|:--|--:|--:|--:|--:|--:|--:|
| swift/small/snapkit-constraint-item.swift | 15.8 | 18.1 | 122.6 | 137.5 | 648.0 | 704.6 |

github-dark from JSON: median 108.4 µs, p90 116.9 µs

## 1f9d2a72, ruby

| file | new, median | new, p90 | first line, median | first line, p90 | document, median | document, p90 |
|:--|--:|--:|--:|--:|--:|--:|
| ruby/small/faraday-middleware.rb | 16.7 | 27.0 | 113.4 | 124.0 | 523.1 | 554.0 |

github-dark from JSON: median 108.4 µs, p90 125.5 µs
