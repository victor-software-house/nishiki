L # ![Rack](contrib/logo.webp)
T 1 text.html.markdown markup.heading.markdown heading.1.markdown punctuation.definition.heading.markdown
T 2 text.html.markdown markup.heading.markdown heading.1.markdown
T 4 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown meta.image.inline.markdown punctuation.definition.link.description.begin.markdown
T 8 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown meta.image.inline.markdown string.other.link.description.markdown
T 9 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown meta.image.inline.markdown punctuation.definition.link.description.end.markdown
T 10 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 27 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown meta.image.inline.markdown markup.underline.link.image.markdown
T 28 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
L 
L Rack provides a minimal, modular, and adaptable interface for developing web
T 76 text.html.markdown meta.paragraph.markdown
L applications in Ruby. By wrapping HTTP requests and responses in the simplest
T 77 text.html.markdown meta.paragraph.markdown
L way possible, it unifies and distills the bridge between web servers, web
T 73 text.html.markdown meta.paragraph.markdown
L frameworks, and web application into a single method call.
T 58 text.html.markdown meta.paragraph.markdown
L 
L The exact details of this are described in the [Rack Specification], which all
T 78 text.html.markdown meta.paragraph.markdown
L Rack applications should conform to. Browse the [Documentation] for more
T 48 text.html.markdown meta.paragraph.markdown
T 49 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 72 text.html.markdown meta.paragraph.markdown
L information.
T 12 text.html.markdown meta.paragraph.markdown
L 
L ## Version support
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 18 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L | Version  | Support                                     |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 2 text.html.markdown markup.table.markdown
T 9 text.html.markdown markup.table.markdown
T 11 text.html.markdown markup.table.markdown
T 12 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 13 text.html.markdown markup.table.markdown
T 20 text.html.markdown markup.table.markdown
T 57 text.html.markdown markup.table.markdown
T 58 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L |----------|---------------------------------------------|
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 11 text.html.markdown markup.table.markdown punctuation.separator.table.markdown
T 12 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 57 text.html.markdown markup.table.markdown punctuation.separator.table.markdown
T 58 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L |    3.2.x | Bug fixes and security patches.             |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 5 text.html.markdown markup.table.markdown
T 10 text.html.markdown markup.table.markdown
T 11 text.html.markdown markup.table.markdown
T 12 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 13 text.html.markdown markup.table.markdown
T 44 text.html.markdown markup.table.markdown
T 57 text.html.markdown markup.table.markdown
T 58 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L |    3.1.x | Security patches only.                      |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 5 text.html.markdown markup.table.markdown
T 10 text.html.markdown markup.table.markdown
T 11 text.html.markdown markup.table.markdown
T 12 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 13 text.html.markdown markup.table.markdown
T 35 text.html.markdown markup.table.markdown
T 57 text.html.markdown markup.table.markdown
T 58 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L |    3.0.x | End of support.                             |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 5 text.html.markdown markup.table.markdown
T 10 text.html.markdown markup.table.markdown
T 11 text.html.markdown markup.table.markdown
T 12 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 13 text.html.markdown markup.table.markdown
T 28 text.html.markdown markup.table.markdown
T 57 text.html.markdown markup.table.markdown
T 58 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L |    2.2.x | Security patches only (through April 2027). |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 5 text.html.markdown markup.table.markdown
T 10 text.html.markdown markup.table.markdown
T 11 text.html.markdown markup.table.markdown
T 12 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 13 text.html.markdown markup.table.markdown
T 56 text.html.markdown markup.table.markdown
T 57 text.html.markdown markup.table.markdown
T 58 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L | <= 2.1.x | End of support.                             |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 2 text.html.markdown markup.table.markdown
T 3 text.html.markdown markup.table.markdown meta.other.valid-bracket.markdown
T 10 text.html.markdown markup.table.markdown
T 11 text.html.markdown markup.table.markdown
T 12 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 13 text.html.markdown markup.table.markdown
T 28 text.html.markdown markup.table.markdown
T 57 text.html.markdown markup.table.markdown
T 58 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L 
L **Rack 2.2.x is in security maintenance mode, and all support will end in May
T 77 text.html.markdown meta.paragraph.markdown
L 2027**. Please upgrade to Rack 3.2 as soon as possible to ensure you are receiving
T 82 text.html.markdown meta.paragraph.markdown
L the latest bug fixes and security patches.
T 42 text.html.markdown meta.paragraph.markdown
L 
L Please see the [Security Policy] for more information.
T 54 text.html.markdown meta.paragraph.markdown
L 
L ## Change log
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 13 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L See the [Changelog](CHANGELOG.md) for a detailed list of changes in each version of Rack.
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 89 text.html.markdown meta.paragraph.markdown
L 
L ### Rack 3.2 (latest release)
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 29 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L This version of rack is supported with bug fixes and security patches.
T 70 text.html.markdown meta.paragraph.markdown
L 
L ### Rack 3.1
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 12 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L This version of rack is supported with security patches only.
T 61 text.html.markdown meta.paragraph.markdown
L 
L ### Rack 3.0
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 12 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L This version of rack is no longer supported. It contains significant changes
T 76 text.html.markdown meta.paragraph.markdown
L which are detailed in the [Upgrade Guide](UPGRADE-GUIDE.md).
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 60 text.html.markdown meta.paragraph.markdown
L 
L ### Rack 2.2
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 12 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L This version of Rack is receiving security patches only, and effort should be
T 77 text.html.markdown meta.paragraph.markdown
L made to move to Rack 3.2.
T 25 text.html.markdown meta.paragraph.markdown
L 
L Starting in Ruby 3.4 the `base64` dependency will no longer be a default gem,
T 25 text.html.markdown meta.paragraph.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 33 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown meta.paragraph.markdown
L and may cause a warning or error about `base64` being missing. To correct this,
T 39 text.html.markdown meta.paragraph.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown
L add `base64` as a dependency to your project.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 12 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown
L 
L ## Installation
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 15 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Add the rack gem to your application bundle, or follow the instructions provided
T 80 text.html.markdown meta.paragraph.markdown
L by a [supported web framework](#supported-web-frameworks):
T 5 text.html.markdown meta.paragraph.markdown
T 6 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 58 text.html.markdown meta.paragraph.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L # Install it generally:
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L $ gem install rack
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L 
L # or, add it to your current application gemfile:
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L $ bundle add rack
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L If you need features from `Rack::Session` or `bin/rackup` please add those gems separately.
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 91 text.html.markdown meta.paragraph.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L $ gem install rack-session rackup
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Usage
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 8 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Create a file called `config.ru` with the following contents:
T 21 text.html.markdown meta.paragraph.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L run do |env|
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.start-block.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby variable.other.block.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
L   [200, {}, ["Hello World"]]
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.begin.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.numeric.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.scope.begin.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.scope.end.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.begin.ruby
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.double.interpolated.ruby punctuation.definition.string.begin.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.double.interpolated.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.double.interpolated.ruby punctuation.definition.string.end.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.end.ruby
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.end.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Run this using the rackup gem or another [supported web
T 55 text.html.markdown meta.paragraph.markdown
L server](#supported-web-servers).
T 32 text.html.markdown meta.paragraph.markdown
L 
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L $ gem install rackup
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L $ rackup
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L 
L # In another shell:
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L $ curl http://localhost:9292
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L Hello World
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Supported web servers
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 24 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Rack is supported by a wide range of servers, including:
T 56 text.html.markdown meta.paragraph.markdown
L 
L * [Agoo](https://github.com/ohler55/agoo)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Falcon](https://github.com/socketry/falcon)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Iodine](https://github.com/boazsegev/iodine)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [NGINX Unit](https://unit.nginx.org/)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Phusion Passenger](https://www.phusionpassenger.com/) (which is mod_rack for
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 56 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 79 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   Apache and for nginx)
T 2 text.html.markdown markup.list.unnumbered.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * [Pitchfork](https://github.com/Shopify/pitchfork)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 51 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Puma](https://puma.io/)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Thin](https://github.com/macournoyer/thin)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Unicorn](https://yhbt.net/unicorn/)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [uWSGI](https://uwsgi-docs.readthedocs.io/en/latest/)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Lamby](https://lamby.custominktech.com) (for AWS Lambda)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 59 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L You will need to consult the server documentation to find out what features and
T 79 text.html.markdown meta.paragraph.markdown
L limitations they may have. In general, any valid Rack app will run the same on
T 78 text.html.markdown meta.paragraph.markdown
L all these servers, without changing anything.
T 45 text.html.markdown meta.paragraph.markdown
L 
L ### Rackup
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 10 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Rack provides a separate gem, [rackup](https://github.com/rack/rackup) which is
T 30 text.html.markdown meta.paragraph.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 69 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 79 text.html.markdown meta.paragraph.markdown
L a generic interface for running a Rack application on supported servers, which
T 78 text.html.markdown meta.paragraph.markdown
L include `WEBRick`, `Puma`, `Falcon` and others.
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 27 text.html.markdown meta.paragraph.markdown
T 28 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown meta.paragraph.markdown
L 
L ## Supported web frameworks
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 27 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L These frameworks and many others support the [Rack Specification]:
T 66 text.html.markdown meta.paragraph.markdown
L 
L * [Camping](https://github.com/camping/camping)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Hanami](https://hanamirb.org/)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Ramaze](https://github.com/ramaze/ramaze)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Padrino](https://padrinorb.com/)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Roda](https://github.com/jeremyevans/roda)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Ruby on Rails](https://rubyonrails.org/)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Rum](https://github.com/leahneukirchen/rum)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Sinatra](https://sinatrarb.com/)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [Utopia](https://github.com/socketry/utopia)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * [WABuR](https://github.com/ohler55/wabur)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ## Available middleware shipped with Rack
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 41 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Between the server and the framework, Rack can be customized to your
T 68 text.html.markdown meta.paragraph.markdown
L applications needs using middleware. Rack itself ships with the following
T 73 text.html.markdown meta.paragraph.markdown
L middleware:
T 11 text.html.markdown meta.paragraph.markdown
L 
L * `Rack::CommonLogger` for creating Apache-style logfiles.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::ConditionalGet` for returning [Not
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   Modified](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/304)
T 2 text.html.markdown markup.list.unnumbered.markdown
T 73 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   responses when the response has not changed.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Config` for modifying the environment before processing the request.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::ContentLength` for setting a `content-length` header based on body
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   size.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::ContentType` for setting a default `content-type` header for responses.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 57 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 58 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Deflater` for compressing responses with gzip.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::ETag` for setting `etag` header on bodies that can be buffered.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Events` for providing easy hooks when a request is received and when
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   the response is sent.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Head` for returning an empty body for HEAD requests.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Lint` for checking conformance to the [Rack Specification].
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Lock` for serializing requests using a mutex.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::MethodOverride` for modifying the request method based on a submitted
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   parameter.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Recursive` for including data from other paths in the application, and
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   for performing internal redirects.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Reloader` for reloading files if they have been modified.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 66 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Runtime` for including a response header with the time taken to process
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   the request.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Sendfile` for working with web servers that can use optimized file
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   serving for file system paths.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::ShowException` for catching unhandled exceptions and presenting them in
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   a nice and helpful way with clickable backtrace.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::ShowStatus` for using nice error pages for empty client error
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 70 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   responses.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Static` for configurable serving of static files.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::TempfileReaper` for removing temporary files creating during a request.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L All these components use the same interface, which is described in detail in the
T 80 text.html.markdown meta.paragraph.markdown
L [Rack Specification]. These optional components can be used in any way you wish.
T 80 text.html.markdown meta.paragraph.markdown
L 
L ### Convenience interfaces
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 26 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If you want to develop outside of existing frameworks, implement your own ones,
T 79 text.html.markdown meta.paragraph.markdown
L or develop middleware, Rack provides many helpers to create Rack applications
T 77 text.html.markdown meta.paragraph.markdown
L quickly and without doing the same web stuff all over:
T 54 text.html.markdown meta.paragraph.markdown
L 
L * `Rack::Request` which also provides query string parsing and multipart
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   handling.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Response` for convenient generation of HTTP replies and cookie
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 71 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   handling.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::MockRequest` and `Rack::MockResponse` for efficient and quick testing
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   of Rack application without real HTTP round-trips.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 52 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Cascade` for trying additional Rack applications if an application
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   returns a not found or method not supported response.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Directory` for serving files under a given directory, with directory
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   indexes.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Files` for serving files under a given directory, without directory
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   indexes.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::MediaType` for parsing content-type headers.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::Mime` for determining content-type based on file extension.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::RewindableInput` for making any IO object rewindable, using a temporary
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   file buffer.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `Rack::URLMap` to route to multiple applications inside the same process.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ## Configuration
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 16 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Rack exposes several configuration parameters to control various features of the
T 80 text.html.markdown meta.paragraph.markdown
L implementation.
T 15 text.html.markdown meta.paragraph.markdown
L 
L ### `RACK_QUERY_PARSER_BYTESIZE_LIMIT`
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 5 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 38 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L This environment variable sets the default for the maximum query string bytesize
T 80 text.html.markdown meta.paragraph.markdown
L that `Rack::QueryParser` will attempt to parse.  Attempts to use a query string
T 5 text.html.markdown meta.paragraph.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown
L or application/x-www-form-urlencoded request body that exceeds this number of
T 77 text.html.markdown meta.paragraph.markdown
L bytes will result in a `Rack::QueryParser::QueryLimitError` exception. If this
T 23 text.html.markdown meta.paragraph.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown meta.paragraph.markdown
L enviroment variable is provided, it must be an integer, or `Rack::QueryParser`
T 59 text.html.markdown meta.paragraph.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 78 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L will raise an exception.
T 24 text.html.markdown meta.paragraph.markdown
L If the value is negative, the bytesize limit is disabled.
T 57 text.html.markdown meta.paragraph.markdown
L 
L The default limit can be overridden on a per-`Rack::QueryParser` basis using
T 45 text.html.markdown meta.paragraph.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 63 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 64 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown
L the `bytesize_limit` keyword argument when creating the `Rack::QueryParser`.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown meta.paragraph.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 74 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 75 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown
L Passing `nil` for this keyword argument disables the bytesize limit.
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 12 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 13 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown
L 
L ### `RACK_QUERY_PARSER_PARAMS_LIMIT`
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 5 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L This environment variable sets the default for the maximum number of query
T 74 text.html.markdown meta.paragraph.markdown
L parameters that `Rack::QueryParser` will attempt to parse.  Attempts to use a
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown meta.paragraph.markdown
L query string or application/x-www-form-urlencoded request body with more
T 72 text.html.markdown meta.paragraph.markdown
L parameters than this will result in a `Rack::QueryParser::QueryLimitError`
T 38 text.html.markdown meta.paragraph.markdown
T 39 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 74 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L exception. If this enviroment variable is provided, it must be an integer, or
T 77 text.html.markdown meta.paragraph.markdown
L `Rack::QueryParser` will raise an exception.
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown meta.paragraph.markdown
L If the value is negative, the params limit is disabled.
T 55 text.html.markdown meta.paragraph.markdown
L 
L The default limit can be overridden on a per-`Rack::QueryParser` basis using
T 45 text.html.markdown meta.paragraph.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 63 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 64 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown
L the `params_limit` keyword argument when creating the `Rack::QueryParser`.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown meta.paragraph.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 73 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 74 text.html.markdown meta.paragraph.markdown
L Passing `nil` for this keyword argument disables the params limit.
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 12 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 13 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 66 text.html.markdown meta.paragraph.markdown
L 
L This is implemented by counting the number of parameter separators in the
T 73 text.html.markdown meta.paragraph.markdown
L query string, before attempting parsing, so if the same parameter key is
T 72 text.html.markdown meta.paragraph.markdown
L used multiple times in the query, each counts as a separate parameter for
T 73 text.html.markdown meta.paragraph.markdown
L this check.
T 11 text.html.markdown meta.paragraph.markdown
L 
L ### `RACK_MULTIPART_BUFFERED_UPLOAD_BYTESIZE_LIMIT`
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 5 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 51 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L This environment variable sets the maximum amount of memory Rack will use
T 73 text.html.markdown meta.paragraph.markdown
L to buffer multipart parameters when parsing a request body. This considers
T 74 text.html.markdown meta.paragraph.markdown
L the size of the multipart mime headers and the body part for multipart
T 70 text.html.markdown meta.paragraph.markdown
L parameters that are buffered in memory and do not use tempfiles. This
T 69 text.html.markdown meta.paragraph.markdown
L defaults to 16MB if not provided.
T 33 text.html.markdown meta.paragraph.markdown
L 
L ### `param_depth_limit`
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 5 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 22 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 23 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L Rack::Utils.param_depth_limit = 32 # default
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.operator.assignment.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.numeric.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The maximum amount of nesting allowed in parameters. For example, if set to 3,
T 78 text.html.markdown meta.paragraph.markdown
L this query string would be allowed:
T 35 text.html.markdown meta.paragraph.markdown
L 
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L ?a[b][c]=d
T 10 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L but this query string would not be allowed:
T 43 text.html.markdown meta.paragraph.markdown
L 
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L ?a[b][c][d]=e
T 13 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Limiting the depth prevents a possible stack overflow when parsing parameters.
T 78 text.html.markdown meta.paragraph.markdown
L 
L ### `multipart_file_limit`
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 5 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 26 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L Rack::Utils.multipart_file_limit = 128 # default
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.operator.assignment.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.numeric.ruby
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The maximum number of parts with a filename a request can contain. Accepting
T 76 text.html.markdown meta.paragraph.markdown
L too many parts can lead to the server running out of file handles.
T 66 text.html.markdown meta.paragraph.markdown
L 
L The default is 128, which means that a single request can't upload more than 128
T 80 text.html.markdown meta.paragraph.markdown
L files at once. Set to 0 for no limit.
T 37 text.html.markdown meta.paragraph.markdown
L 
L Can also be set via the `RACK_MULTIPART_FILE_LIMIT` environment variable.
T 24 text.html.markdown meta.paragraph.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown meta.paragraph.markdown
L 
L (This is also aliased as `multipart_part_limit` and `RACK_MULTIPART_PART_LIMIT` for compatibility)
T 25 text.html.markdown meta.paragraph.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 79 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 98 text.html.markdown meta.paragraph.markdown
L 
L ### `multipart_total_part_limit`
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 5 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 32 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L The maximum total number of parts a request can contain of any type, including
T 78 text.html.markdown meta.paragraph.markdown
L both file and non-file form fields.
T 35 text.html.markdown meta.paragraph.markdown
L 
L The default is 4096, which means that a single request can't contain more than
T 78 text.html.markdown meta.paragraph.markdown
L 4096 parts.
T 11 text.html.markdown meta.paragraph.markdown
L 
L Set to 0 for no limit.
T 22 text.html.markdown meta.paragraph.markdown
L 
L Can also be set via the `RACK_MULTIPART_TOTAL_PART_LIMIT` environment variable.
T 24 text.html.markdown meta.paragraph.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown
L 
L ## Contributing
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 15 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L See [CONTRIBUTING.md](CONTRIBUTING.md) for specific details about how to make a
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 79 text.html.markdown meta.paragraph.markdown
L contribution to Rack.
T 21 text.html.markdown meta.paragraph.markdown
L 
L Please post bugs, suggestions and patches to [GitHub Issues](https://github.com/rack/rack/issues).
T 45 text.html.markdown meta.paragraph.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 96 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 97 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 98 text.html.markdown meta.paragraph.markdown
L 
L Please check our [Security Policy](https://github.com/rack/rack/security/policy)
T 17 text.html.markdown meta.paragraph.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 79 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 80 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L for responsible disclosure and security bug reporting process. Due to wide usage
T 80 text.html.markdown meta.paragraph.markdown
L of the library, it is strongly preferred that we manage timing in order to
T 74 text.html.markdown meta.paragraph.markdown
L provide viable patches at the time of disclosure. Your assistance in this matter
T 80 text.html.markdown meta.paragraph.markdown
L is greatly appreciated.
T 23 text.html.markdown meta.paragraph.markdown
L 
L ## See Also
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 11 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### `rackup`
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 5 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 11 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 12 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L A useful tool for running Rack applications from the command line, including
T 76 text.html.markdown meta.paragraph.markdown
L `Rackup::Server` (previously `Rack::Server`) for scripting servers.
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown meta.paragraph.markdown
L 
L * https://github.com/rack/rackup
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ### `rack-contrib`
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 5 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L The plethora of useful middleware created the need for a project that collects
T 78 text.html.markdown meta.paragraph.markdown
L fresh Rack middleware. `rack-contrib` includes a variety of add-on components
T 23 text.html.markdown meta.paragraph.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown meta.paragraph.markdown
L for Rack and it is easy to contribute new modules.
T 50 text.html.markdown meta.paragraph.markdown
L 
L * https://github.com/rack/rack-contrib
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ### `rack-session`
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 5 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L Provides convenient session management for Rack.
T 48 text.html.markdown meta.paragraph.markdown
L 
L * https://github.com/rack/rack-session
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ## Thanks
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 9 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L The Rack Core Team, consisting of
T 33 text.html.markdown meta.paragraph.markdown
L 
L * Aaron Patterson [tenderlove](https://github.com/tenderlove)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 60 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * Samuel Williams [ioquatix](https://github.com/ioquatix)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 56 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 57 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * Jeremy Evans [jeremyevans](https://github.com/jeremyevans)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 59 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 60 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * Eileen Uchitelle [eileencodes](https://github.com/eileencodes)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 63 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 64 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * Matthew Draper [matthewd](https://github.com/matthewd)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 56 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * Rafael França [rafaelfranca](https://github.com/rafaelfranca)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 63 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 64 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L and the Rack Alumni
T 19 text.html.markdown meta.paragraph.markdown
L 
L * Ryan Tomayko [rtomayko](https://github.com/rtomayko)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 53 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * Scytrin dai Kinthra [scytrin](https://github.com/scytrin)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 58 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 59 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * Leah Neukirchen [leahneukirchen](https://github.com/leahneukirchen)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * James Tucker [raggi](https://github.com/raggi)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 48 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * Josh Peek [josh](https://github.com/josh)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * José Valim [josevalim](https://github.com/josevalim)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * Michael Fellinger [manveru](https://github.com/manveru)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 56 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 57 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * Santiago Pastorino [spastorino](https://github.com/spastorino)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 63 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 64 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L * Konstantin Haase [rkh](https://github.com/rkh)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 48 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L would like to thank:
T 20 text.html.markdown meta.paragraph.markdown
L 
L * Adrian Madrid, for the LiteSpeed handler.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Christoffer Sawicki, for the first Rails adapter and `Rack::Deflater`.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 56 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 70 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 71 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Tim Fletcher, for the HTTP authentication code.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 49 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Luc Heinrich for the Cookie sessions, the static file handler and bugfixes.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 77 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Armin Ronacher, for the logo and racktools.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Alex Beregszaszi, Alexander Kahn, Anil Wadghule, Aredridel, Ben Alpert, Dan
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 77 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   Kubb, Daniel Roethlisberger, Matt Todd, Tom Robinson, Phil Hagelberg, S. Brent
T 2 text.html.markdown markup.list.unnumbered.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   Faulkner, Bosko Milekic, Daniel Rodríguez Troitiño, Genki Takiuchi, Geoffrey
T 2 text.html.markdown markup.list.unnumbered.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   Grosenbach, Julien Sanchez, Kamal Fariz Mahyuddin, Masayoshi Takahashi,
T 2 text.html.markdown markup.list.unnumbered.markdown
T 73 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   Patrick Aljordm, Mig, Kazuhiro Nishiyama, Jon Bardin, Konstantin Haase, Larry
T 2 text.html.markdown markup.list.unnumbered.markdown
T 79 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   Siden, Matias Korhonen, Sam Ruby, Simon Chiang, Tim Connor, Timur Batyrshin,
T 2 text.html.markdown markup.list.unnumbered.markdown
T 78 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   and Zach Brock for bug fixing and other improvements.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Eric Wong, Hongli Lai, Jeremy Kemper for their continuous support and API
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 75 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   improvements.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Yehuda Katz and Carl Lerche for refactoring rackup.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 53 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Brian Candler, for `Rack::ContentType`.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Graham Batty, for improved handler loading.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Stephen Bannasch, for bug reports and documentation.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Gary Wright, for proposing a better `Rack::Response` interface.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Jonathan Buch, for improvements regarding `Rack::Response`.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 60 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Armin Röhrl, for tracking down bugs in the Cookie generator.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 63 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Alexander Kellett for testing the Gem and reviewing the announcement.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 71 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Marcus Rückert, for help with configuring and debugging lighttpd.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * The WSGI team for the well-done and documented work they've done and Rack
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 75 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   builds up on.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * All bug reporters and patch contributors not mentioned above.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 63 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ## License
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 10 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Rack is released under the [MIT License](MIT-LICENSE).
T 27 text.html.markdown meta.paragraph.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 52 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 54 text.html.markdown meta.paragraph.markdown
L 
L [Rack Specification]: https://rack.github.io/rack/main/SPEC_rdoc.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 19 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 20 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 21 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 22 text.html.markdown meta.link.reference.def.markdown
T 69 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [Documentation]: https://rack.github.io/rack/
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 17 text.html.markdown meta.link.reference.def.markdown
T 45 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [Security Policy]: SECURITY.md
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 17 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 18 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 19 text.html.markdown meta.link.reference.def.markdown
T 30 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
