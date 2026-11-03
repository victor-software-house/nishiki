L ![Devise Logo](https://raw.github.com/heartcombo/devise/main/devise.png)
T 2 text.html.markdown meta.paragraph.markdown meta.image.inline.markdown punctuation.definition.link.description.begin.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.image.inline.markdown string.other.link.description.markdown
T 14 text.html.markdown meta.paragraph.markdown meta.image.inline.markdown punctuation.definition.link.description.end.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.image.inline.markdown markup.underline.link.image.markdown
T 72 text.html.markdown meta.paragraph.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
L 
L Devise is a flexible authentication solution for Rails based on Warden. It:
T 75 text.html.markdown meta.paragraph.markdown
L 
L * Is Rack based;
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Is a complete MVC solution based on Rails engines;
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 52 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Allows you to have multiple models signed in at the same time;
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 64 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Is based on a modularity concept: use only what you really need.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 66 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L It's composed of 10 modules:
T 28 text.html.markdown meta.paragraph.markdown
L 
L * [Database Authenticatable](https://www.rubydoc.info/gems/devise/Devise/Models/DatabaseAuthenticatable): hashes and stores a password in the database to validate the authenticity of a user while signing in. The authentication can be done both through POST requests or HTTP Basic Authentication.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 103 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 104 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 295 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * [Omniauthable](https://www.rubydoc.info/gems/devise/Devise/Models/Omniauthable): adds OmniAuth (https://github.com/omniauth/omniauth) support.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 81 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 144 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * [Confirmable](https://www.rubydoc.info/gems/devise/Devise/Models/Confirmable): sends emails with confirmation instructions and verifies whether an account is already confirmed during sign in.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 78 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 79 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 193 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * [Recoverable](https://www.rubydoc.info/gems/devise/Devise/Models/Recoverable): resets the user password and sends reset instructions.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 78 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 79 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 135 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * [Registerable](https://www.rubydoc.info/gems/devise/Devise/Models/Registerable): handles signing up users through a registration process, also allowing them to edit and destroy their account.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 81 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 193 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * [Rememberable](https://www.rubydoc.info/gems/devise/Devise/Models/Rememberable): manages generating and clearing a token for remembering the user from a saved cookie.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 81 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 168 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * [Trackable](https://www.rubydoc.info/gems/devise/Devise/Models/Trackable): tracks sign in count, timestamps and IP address.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 74 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 75 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 125 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * [Timeoutable](https://www.rubydoc.info/gems/devise/Devise/Models/Timeoutable): expires sessions that have not been active in a specified period of time.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 78 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 79 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 154 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * [Validatable](https://www.rubydoc.info/gems/devise/Devise/Models/Validatable): provides validations of email and password. It's optional and can be customized, so you're able to define your own validations.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 78 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 79 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 208 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * [Lockable](https://www.rubydoc.info/gems/devise/Devise/Models/Lockable): locks an account after a specified number of failed sign-in attempts. Can unlock via email or after a specified time period.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 72 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 73 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 199 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ## Table of Contents
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 20 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L <!-- TOC depthFrom:1 depthTo:6 withLinks:1 orderedList:0 -->
T 4 text.html.markdown comment.block.html punctuation.definition.comment.html
T 57 text.html.markdown comment.block.html
T 60 text.html.markdown comment.block.html punctuation.definition.comment.html
L 
L - [Information](#information)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [The Devise wiki](#the-devise-wiki)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 38 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Bug reports](#bug-reports)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [StackOverflow and Mailing List](#stackoverflow-and-mailing-list)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 67 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [RDocs](#rdocs)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Example applications](#example-applications)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 48 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Extensions](#extensions)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Supported Ruby / Rails versions](#supported-ruby--rails-versions)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Contributing](#contributing)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Starting with Rails?](#starting-with-rails)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 45 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Getting started](#getting-started)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Controller filters and helpers](#controller-filters-and-helpers)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 67 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Configuring Models](#configuring-models)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Strong Parameters](#strong-parameters)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Configuring views](#configuring-views)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Configuring controllers](#configuring-controllers)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 53 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Configuring routes](#configuring-routes)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [I18n](#i18n)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Test helpers](#test-helpers)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Controller tests](#controller-tests)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 39 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Integration tests](#integration-tests)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [OmniAuth](#omniauth)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 23 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Configuring multiple models](#configuring-multiple-models)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 33 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Active Job Integration](#active-job-integration)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 51 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 52 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Password reset tokens and Rails logs](#password-reset-tokens-and-rails-logs)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 42 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 79 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Other ORMs](#other-orms)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 28 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Rails API mode](#rails-api-mode)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 36 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [Additional information](#additional-information)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 25 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 50 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 51 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 	- [Warden](#warden)
T 1 text.html.markdown markup.list.unnumbered.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown
T 4 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L - [License](#license)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 12 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L <!-- /TOC -->
T 4 text.html.markdown comment.block.html punctuation.definition.comment.html
T 10 text.html.markdown comment.block.html
T 13 text.html.markdown comment.block.html punctuation.definition.comment.html
L 
L 
L 
L ## Information
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 14 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### The Devise wiki
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 19 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L The Devise Wiki has lots of additional information about Devise including many "how-to" articles and answers to the most frequently asked questions. Please browse the Wiki after finishing this README:
T 200 text.html.markdown meta.paragraph.markdown
L 
L https://github.com/heartcombo/devise/wiki
T 41 text.html.markdown meta.paragraph.markdown
L 
L ### Bug reports
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 15 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If you discover a problem with Devise, we would like to know about it. However, we ask that you please review these guidelines before submitting a bug report:
T 158 text.html.markdown meta.paragraph.markdown
L 
L https://github.com/heartcombo/devise/wiki/Bug-reports
T 53 text.html.markdown meta.paragraph.markdown
L 
L If you have discovered a security related bug, please do *NOT* use the GitHub issue tracker. Send an email to heartcombo.oss@gmail.com.
T 57 text.html.markdown meta.paragraph.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 62 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 135 text.html.markdown meta.paragraph.markdown
L 
L ### StackOverflow and Mailing List
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 34 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If you have any questions, comments, or concerns, please use StackOverflow instead of the GitHub issue tracker:
T 111 text.html.markdown meta.paragraph.markdown
L 
L https://stackoverflow.com/questions/tagged/devise
T 49 text.html.markdown meta.paragraph.markdown
L 
L The deprecated mailing lists can still be read on:
T 50 text.html.markdown meta.paragraph.markdown
L 
L https://groups.google.com/group/plataformatec-devise
T 52 text.html.markdown meta.paragraph.markdown
L https://groups.google.com/group/heartcombo
T 42 text.html.markdown meta.paragraph.markdown
L 
L ### RDocs
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 9 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L You can view the Devise documentation in RDoc format here:
T 58 text.html.markdown meta.paragraph.markdown
L 
L https://rubydoc.info/github/heartcombo/devise/main/frames
T 57 text.html.markdown meta.paragraph.markdown
L 
L If you need to use Devise with previous versions of Rails, you can always run "gem server" from the command line after you install the gem to access the old documentation.
T 171 text.html.markdown meta.paragraph.markdown
L 
L ### Example applications
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 24 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L There are a few example applications available on GitHub that demonstrate various features of Devise with different versions of Rails. You can view them here:
T 158 text.html.markdown meta.paragraph.markdown
L 
L https://github.com/heartcombo/devise/wiki/Example-Applications
T 62 text.html.markdown meta.paragraph.markdown
L 
L ### Extensions
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 14 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Our community has created a number of extensions that add functionality above and beyond what is included with Devise. You can view a list of available extensions and add your own here:
T 185 text.html.markdown meta.paragraph.markdown
L 
L https://github.com/heartcombo/devise/wiki/Extensions
T 52 text.html.markdown meta.paragraph.markdown
L 
L ### Supported Ruby / Rails versions
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 35 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L We intend to maintain support for all Ruby / Rails versions that haven't reached end-of-life.
T 93 text.html.markdown meta.paragraph.markdown
L 
L For more information about specific versions please check [Ruby](https://www.ruby-lang.org/en/downloads/branches/)
T 58 text.html.markdown meta.paragraph.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 113 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 114 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L and [Rails](https://guides.rubyonrails.org/maintenance_policy.html) maintenance policies, and our test matrix.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 12 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 66 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 110 text.html.markdown meta.paragraph.markdown
L 
L ### Contributing
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 16 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L We hope that you will consider contributing to Devise. Please read this short overview for some information about how to get started:
T 133 text.html.markdown meta.paragraph.markdown
L 
L https://github.com/heartcombo/devise/wiki/Contributing
T 54 text.html.markdown meta.paragraph.markdown
L 
L You will usually want to write tests for your changes.  To run the test suite, go into Devise's top-level directory and run `bundle install` and `bin/test`.
T 124 text.html.markdown meta.paragraph.markdown
T 125 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 139 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 140 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 145 text.html.markdown meta.paragraph.markdown
T 146 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 154 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 155 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 156 text.html.markdown meta.paragraph.markdown
L Devise works with multiple Ruby and Rails versions, and ActiveRecord and Mongoid ORMs, which means you can run the test suite with some modifiers: `DEVISE_ORM` and `BUNDLE_GEMFILE`.
T 147 text.html.markdown meta.paragraph.markdown
T 148 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 158 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 159 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 164 text.html.markdown meta.paragraph.markdown
T 165 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 179 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 180 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 181 text.html.markdown meta.paragraph.markdown
L 
L #### DEVISE_ORM
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 15 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L Since Devise supports both Mongoid and ActiveRecord, we rely on this variable to run specific code for each ORM.
T 112 text.html.markdown meta.paragraph.markdown
L The default value of `DEVISE_ORM` is `active_record`. To run the tests for Mongoid, you can pass `mongoid`:
T 21 text.html.markdown meta.paragraph.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 33 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown meta.paragraph.markdown
T 38 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 97 text.html.markdown meta.paragraph.markdown
T 98 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 105 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 106 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 107 text.html.markdown meta.paragraph.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L DEVISE_ORM=mongoid bin/test
T 27 text.html.markdown markup.fenced_code.block.markdown
L 
L ==> Devise.orm = :mongoid
T 25 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L When running the tests for Mongoid, you will need to have a MongoDB server (version 2.0 or newer) running on your system.
T 121 text.html.markdown meta.paragraph.markdown
L 
L Please note that the command output will show the variable value being used.
T 76 text.html.markdown meta.paragraph.markdown
L 
L #### BUNDLE_GEMFILE
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 19 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L We can use this variable to tell bundler what Gemfile it should use (instead of the one in the current directory).
T 114 text.html.markdown meta.paragraph.markdown
L Inside the [gemfiles](https://github.com/heartcombo/devise/tree/main/gemfiles) directory, we have one for each version of Rails we support. When you send us a pull request, it may happen that the test suite breaks using some of them. If that's the case, you can simulate the same environment using the `BUNDLE_GEMFILE` variable.
T 11 text.html.markdown meta.paragraph.markdown
T 12 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 77 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 78 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 302 text.html.markdown meta.paragraph.markdown
T 303 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 317 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 318 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 328 text.html.markdown meta.paragraph.markdown
L For example, if the tests broke using Ruby 3.4 and Rails 8.0, you can do the following:
T 87 text.html.markdown meta.paragraph.markdown
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L chruby 3.4.0 # or rbenv shell 3.4.0, or rvm use 3.4.0, etc.
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell constant.numeric.shell constant.numeric.version.shell
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell punctuation.definition.comment.shell
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript comment.line.number-sign.shell
L BUNDLE_GEMFILE=gemfiles/Gemfile-rails-8-0 bundle install
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell variable.other.assignment.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell keyword.operator.assignment.shell
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell meta.argument.shell
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell meta.argument.shell string.unquoted.argument.shell
L BUNDLE_GEMFILE=gemfiles/Gemfile-rails-8-0 bin/test
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell variable.other.assignment.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell keyword.operator.assignment.shell
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can also combine both of them if the tests broke for Mongoid:
T 65 text.html.markdown meta.paragraph.markdown
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L BUNDLE_GEMFILE=gemfiles/Gemfile-rails-8-0 bundle install
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell variable.other.assignment.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell keyword.operator.assignment.shell
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell meta.argument.shell
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.statement.command.env.shell meta.argument.shell string.unquoted.argument.shell
L BUNDLE_GEMFILE=gemfiles/Gemfile-rails-8-0 DEVISE_ORM=mongoid bin/test
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell variable.other.assignment.shell
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell keyword.operator.assignment.shell
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell string.unquoted.shell
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell variable.other.assignment.shell
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell keyword.operator.assignment.shell
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell string.unquoted.shell
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell string.unquoted.shell
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell string.unquoted.shell
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell string.unquoted.shell
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell string.unquoted.shell
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell string.unquoted.shell
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell string.unquoted.shell
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.expression.assignment.shell meta.expression.assignment.shell meta.statement.command.env.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Running tests
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 17 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L Devise uses [minitest](https://github.com/seattlerb/minitest) as test framework.
T 12 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 80 text.html.markdown meta.paragraph.markdown
L 
L * Running all tests:
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L bin/test
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L * Running tests for an specific file:
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L bin/test test/models/trackable_test.rb
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L * Running a specific test given a line number or a regex:
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 57 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L ```bash
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L bin/test test/models/trackable_test.rb:16
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L bin/test test/models/trackable_test.rb -n '/update.*record/'
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument.shell constant.other.option.dash.shell
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell string.unquoted.argument constant.other.option
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.quoted.single.shell punctuation.definition.string.begin.shell
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.quoted.single.shell
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.quoted.single.shell punctuation.definition.string.end.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Starting with Rails?
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 23 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L If you are building your first Rails application, we recommend you *do not* use Devise. Devise requires a good understanding of the Rails Framework. In such cases, we advise you to start a simple authentication system from scratch. Here's a few resources that should help you get started:
T 67 text.html.markdown meta.paragraph.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 74 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 75 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 288 text.html.markdown meta.paragraph.markdown
L 
L * Michael Hartl's online book: https://www.railstutorial.org/book/modeling_users
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 80 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Ryan Bates' Railscasts: http://railscasts.com/episodes/250-authentication-from-scratch and http://railscasts.com/episodes/250-authentication-from-scratch-revised
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 163 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Codecademy's Ruby on Rails: Authentication and Authorization: https://www.codecademy.com/learn/rails-auth
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 107 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L Once you have solidified your understanding of Rails and authentication mechanisms, we assure you Devise will be very pleasant to work with. :smiley:
T 149 text.html.markdown meta.paragraph.markdown
L 
L ## Getting started
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 18 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Devise 5 works with Rails 7 onwards. Run:
T 41 text.html.markdown meta.paragraph.markdown
L 
L ```sh
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L bundle add devise
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.statement.command.name.shell entity.name.function.call.shell entity.name.command.shell
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.shellscript meta.statement.shell meta.statement.command.shell meta.argument.shell string.unquoted.argument.shell
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Next, you need to run the generator:
T 36 text.html.markdown meta.paragraph.markdown
L 
L ```console
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 10 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L rails generate devise:install
T 29 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L At this point, a number of instructions will appear in the console. Among these instructions, you'll need to set up the default URL options for the Devise mailer in each environment. Here is a possible configuration for `config/environments/development.rb`:
T 220 text.html.markdown meta.paragraph.markdown
T 221 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 255 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 256 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 257 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L config.action_mailer.default_url_options = { host: 'localhost', port: 3000 }
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.operator.assignment.ruby
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.scope.begin.ruby
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.syntax.ruby.start-block
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.numeric.ruby
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.scope.end.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The generator will install an initializer which describes ALL of Devise's configuration options. It is *imperative* that you take a look at it. When you are done, you are ready to add Devise to any of your models using the generator.
T 103 text.html.markdown meta.paragraph.markdown
T 104 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 114 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 115 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 233 text.html.markdown meta.paragraph.markdown
L 
L 
L In the following command you will replace `MODEL` with the class name used for the application’s users (it’s frequently `User` but could also be `Admin`). This will create a model (if one does not exist) and configure it with the default Devise modules. The generator also configures your `config/routes.rb` file to point to the Devise controller.
T 42 text.html.markdown meta.paragraph.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 124 text.html.markdown meta.paragraph.markdown
T 125 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 129 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 130 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 149 text.html.markdown meta.paragraph.markdown
T 150 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 155 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 156 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 293 text.html.markdown meta.paragraph.markdown
T 294 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 310 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 311 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 351 text.html.markdown meta.paragraph.markdown
L 
L ```console
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 10 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L rails generate devise MODEL
T 27 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Next, check the MODEL for any additional configuration options you might want to add, such as confirmable or lockable. If you add an option, be sure to inspect the migration file (created by the generator if your ORM supports them) and uncomment the appropriate section.  For example, if you add the confirmable option in the model, you'll need to uncomment the Confirmable section in the migration.
T 399 text.html.markdown meta.paragraph.markdown
L 
L Then run `rails db:migrate`
T 9 text.html.markdown meta.paragraph.markdown
T 10 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L You should restart your application after changing Devise's configuration options (this includes stopping spring). Otherwise, you will run into strange errors, for example, users being unable to login and route helpers being undefined.
T 235 text.html.markdown meta.paragraph.markdown
L 
L ### Controller filters and helpers
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 34 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Devise will create some helpers to use inside your controllers and views. To set up a controller with user authentication, just add this before_action (assuming your devise model is 'User'):
T 190 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L before_action :authenticate_user!
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L For Rails 5, note that `protect_from_forgery` is no longer prepended to the `before_action` chain, so if you have set `authenticate_user` before `protect_from_forgery`, your request will result in "Can't verify CSRF token authenticity." To resolve this, either change the order in which you call them, or use `protect_from_forgery prepend: true`.
T 23 text.html.markdown meta.paragraph.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown
T 77 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 90 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 91 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 118 text.html.markdown meta.paragraph.markdown
T 119 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 136 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 137 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 145 text.html.markdown meta.paragraph.markdown
T 146 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 166 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 167 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 309 text.html.markdown meta.paragraph.markdown
T 310 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 344 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 345 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 346 text.html.markdown meta.paragraph.markdown
L 
L If your devise model is something other than User, replace "_user" with "_yourmodel". The same logic applies to the instructions below.
T 135 text.html.markdown meta.paragraph.markdown
L 
L To verify if a user is signed in, use the following helper:
T 59 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L user_signed_in?
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L For the current signed-in user, this helper is available:
T 57 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L current_user
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can access the session for this scope:
T 42 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L user_session
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L After signing in a user, confirming the account or updating the password, Devise will look for a scoped root path to redirect to. For instance, when using a `:user` resource, the `user_root_path` will be used if it exists; otherwise, the default `root_path` will be used. This means that you need to set the root inside your routes:
T 157 text.html.markdown meta.paragraph.markdown
T 158 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 163 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 164 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 179 text.html.markdown meta.paragraph.markdown
T 180 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 194 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 195 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 246 text.html.markdown meta.paragraph.markdown
T 247 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 256 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 257 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 332 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L root to: 'home#index'
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can also override `after_sign_in_path_for` and `after_sign_out_path_for` to customize your redirect hooks.
T 22 text.html.markdown meta.paragraph.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 51 text.html.markdown meta.paragraph.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 76 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 110 text.html.markdown meta.paragraph.markdown
L 
L Notice that if your Devise model is called `Member` instead of `User`, for example, then the helpers available are:
T 43 text.html.markdown meta.paragraph.markdown
T 44 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 63 text.html.markdown meta.paragraph.markdown
T 64 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 115 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L before_action :authenticate_member!
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L 
L member_signed_in?
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
L 
L current_member
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
L 
L member_session
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Configuring Models
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 22 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L The Devise method in your models also accepts some options to configure its modules. For example, you can choose the cost of the hashing algorithm with:
T 152 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L devise :database_authenticatable, :registerable, :confirmable, :recoverable, stretches: 13
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 86 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 87 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 88 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 90 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.numeric.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Besides `:stretches`, you can define `:pepper`, `:encryptor`, `:confirm_within`, `:remember_for`, `:timeout_in`, `:unlock_in` among other options. For more details, see the initializer file that was created when you invoked the "devise:install" generator described above. This file is usually located at `/config/initializers/devise.rb`.
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown meta.paragraph.markdown
T 38 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 48 text.html.markdown meta.paragraph.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 62 text.html.markdown meta.paragraph.markdown
T 63 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 79 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 81 text.html.markdown meta.paragraph.markdown
T 82 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 95 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 96 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 98 text.html.markdown meta.paragraph.markdown
T 99 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 110 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 111 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 113 text.html.markdown meta.paragraph.markdown
T 114 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 124 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 125 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 304 text.html.markdown meta.paragraph.markdown
T 305 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 335 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 336 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 337 text.html.markdown meta.paragraph.markdown
L 
L ### Strong Parameters
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 21 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L The Parameter Sanitizer API has changed for Devise 4 :warning:
T 62 text.html.markdown meta.paragraph.markdown
L 
L *For previous Devise versions see https://github.com/heartcombo/devise/tree/3-stable#strong-parameters*
T 1 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 102 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 103 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
L 
L When you customize your own views, you may end up adding new attributes to forms. Rails 4 moved the parameter sanitization from the model to the controller, causing Devise to handle this concern at the controller as well.
T 221 text.html.markdown meta.paragraph.markdown
L 
L There are just three actions in Devise that allow any set of parameters to be passed down to the model, therefore requiring sanitization. Their names and default permitted parameters are:
T 187 text.html.markdown meta.paragraph.markdown
L 
L * `sign_in` (`Devise::SessionsController#create`) - Permits only the authentication keys (like `email`)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 48 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 95 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 96 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 101 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 102 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 103 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * `sign_up` (`Devise::RegistrationsController#create`) - Permits authentication keys plus `password` and `password_confirmation`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 53 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 90 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 91 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 99 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 100 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 105 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 106 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 127 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 128 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L * `account_update` (`Devise::RegistrationsController#update`) - Permits authentication keys plus `password`, `password_confirmation` and `current_password`
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 3 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 60 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 97 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 98 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 106 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 107 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 109 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 110 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 131 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 132 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 137 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 138 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 154 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 155 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L In case you want to permit additional parameters (the lazy way™), you can do so using a simple before action in your `ApplicationController`:
T 119 text.html.markdown meta.paragraph.markdown
T 120 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 141 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 142 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 143 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L class ApplicationController < ActionController::Base
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby keyword.control.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby punctuation.separator.inheritance.ruby
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby punctuation.separator.namespace.ruby
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
L   before_action :configure_permitted_parameters, if: :devise_controller?
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L 
L   protected
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.other.special-method.ruby
L 
L   def configure_permitted_parameters
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby keyword.control.def.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby entity.name.function.ruby
L     devise_parameter_sanitizer.permit(:sign_up, keys: [:username])
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.begin.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.end.ruby
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
L   end
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The above works for any additional fields where the parameters are simple scalar types. If you have nested attributes (say you're using `accepts_nested_attributes_for`), then you will need to tell devise about those nestings and types:
T 136 text.html.markdown meta.paragraph.markdown
T 137 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 166 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 167 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 235 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L class ApplicationController < ActionController::Base
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby keyword.control.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby punctuation.separator.inheritance.ruby
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby punctuation.separator.namespace.ruby
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
L   before_action :configure_permitted_parameters, if: :devise_controller?
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L 
L   protected
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.other.special-method.ruby
L 
L   def configure_permitted_parameters
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby keyword.control.def.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby entity.name.function.ruby
L     devise_parameter_sanitizer.permit(:sign_up, keys: [:first_name, :last_name, address_attributes: [:country, :state, :city, :area, :postal_code]])
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.begin.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 78 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 98 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 99 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 100 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 101 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.begin.ruby
T 102 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 109 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 110 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 111 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 112 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 117 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 118 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 119 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 120 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 124 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 125 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 126 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 127 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 131 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 132 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 133 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 134 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 145 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 146 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.end.ruby
T 147 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.end.ruby
T 148 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
L   end
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Devise allows you to completely change Devise defaults or invoke custom behavior by passing a block:
T 100 text.html.markdown meta.paragraph.markdown
L 
L To permit simple scalar values for username and email, use this
T 63 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L def configure_permitted_parameters
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby keyword.control.def.ruby
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby entity.name.function.ruby
L   devise_parameter_sanitizer.permit(:sign_in) do |user_params|
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.start-block.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby variable.other.block.ruby
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
L     user_params.permit(:username, :email)
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
L   end
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L If you have some checkboxes that express the roles a user may take on registration, the browser will send those selected checkboxes as an array. An array is not one of Strong Parameters' permitted scalars, so we need to configure Devise in the following way:
T 258 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L def configure_permitted_parameters
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby keyword.control.def.ruby
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby entity.name.function.ruby
L   devise_parameter_sanitizer.permit(:sign_up) do |user_params|
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.start-block.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby variable.other.block.ruby
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
L     user_params.permit({ roles: [] }, :email, :password, :password_confirmation)
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.scope.begin.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.syntax.ruby.start-block
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.begin.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.end.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.scope.end.ruby
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
L   end
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L For the list of permitted scalars, and how to declare permitted keys in nested hashes and arrays, see
T 101 text.html.markdown meta.paragraph.markdown
L 
L https://github.com/rails/strong_parameters#nested-parameters
T 60 text.html.markdown meta.paragraph.markdown
L 
L If you have multiple Devise models, you may want to set up a different parameter sanitizer per model. In this case, we recommend inheriting from `Devise::ParameterSanitizer` and adding your own logic:
T 145 text.html.markdown meta.paragraph.markdown
T 146 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 172 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 173 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 200 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L class User::ParameterSanitizer < Devise::ParameterSanitizer
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby keyword.control.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby punctuation.separator.namespace.ruby
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby punctuation.separator.inheritance.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby punctuation.separator.namespace.ruby
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
L   def initialize(*)
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby keyword.control.def.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby entity.name.function.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby punctuation.definition.parameters.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby keyword.operator.arithmetic.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby punctuation.definition.parameters.ruby
L     super
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.pseudo-method.ruby
L     permit(:sign_up, keys: [:username, :email])
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.section.function.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.separator.object.ruby
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.hashkey.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.section.array.begin.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.separator.object.ruby
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.section.array.end.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.section.function.ruby
L   end
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L And then configure your controllers to use it:
T 46 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L class ApplicationController < ActionController::Base
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby keyword.control.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby punctuation.separator.inheritance.ruby
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby punctuation.separator.namespace.ruby
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
L   protected
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.other.special-method.ruby
L 
L   def devise_parameter_sanitizer
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby keyword.control.def.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby entity.name.function.ruby
L     if resource_class == User
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.operator.comparison.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
L       User::ParameterSanitizer.new(User, :user, params)
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.other.special-method.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
L     else
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L       super # Use the default one
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.pseudo-method.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L     end
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L   end
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The example above overrides the permitted parameters for the user to be both `:username` and `:email`. The non-lazy way to configure parameters would be by defining the before filter above in a custom controller. We detail how to configure and customize controllers in some sections below.
T 77 text.html.markdown meta.paragraph.markdown
T 78 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 87 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 88 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 93 text.html.markdown meta.paragraph.markdown
T 94 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 100 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 101 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 289 text.html.markdown meta.paragraph.markdown
L 
L ### Configuring views
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 21 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L We built Devise to help you quickly develop an application that uses authentication. However, we don't want to be in your way when you need to customize it.
T 156 text.html.markdown meta.paragraph.markdown
L 
L Since Devise is an engine, all its views are packaged inside the gem. These views will help you get started, but after some time you may want to change them. If this is the case, you just need to invoke the following generator, and it will copy all views to your application:
T 275 text.html.markdown meta.paragraph.markdown
L 
L ```console
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 10 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L rails generate devise:views
T 27 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L If you have more than one Devise model in your application (such as `User` and `Admin`), you will notice that Devise uses the same views for all models. Fortunately, Devise offers an easy way to customize views. All you need to do is set `config.scoped_views = true` inside the `config/initializers/devise.rb` file.
T 68 text.html.markdown meta.paragraph.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 74 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown
T 80 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 85 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 86 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 238 text.html.markdown meta.paragraph.markdown
T 239 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 265 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 266 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 278 text.html.markdown meta.paragraph.markdown
T 279 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 308 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 309 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 315 text.html.markdown meta.paragraph.markdown
L 
L After doing so, you will be able to have views based on the role like `users/sessions/new` and `admins/sessions/new`. If no view is found within the scope, Devise will use the default view at `devise/sessions/new`. You can also use the generator to generate scoped views:
T 70 text.html.markdown meta.paragraph.markdown
T 71 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 89 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 90 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 95 text.html.markdown meta.paragraph.markdown
T 96 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 115 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 116 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 192 text.html.markdown meta.paragraph.markdown
T 193 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 212 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 213 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 271 text.html.markdown meta.paragraph.markdown
L 
L ```console
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 10 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L rails generate devise:views users
T 33 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L If you would like to generate only a few sets of views, like the ones for the `registerable` and `confirmable` module,
T 78 text.html.markdown meta.paragraph.markdown
T 79 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 91 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 92 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 97 text.html.markdown meta.paragraph.markdown
T 98 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 109 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 110 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 118 text.html.markdown meta.paragraph.markdown
L you can pass a list of views to the generator with the `-v` flag.
T 55 text.html.markdown meta.paragraph.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown meta.paragraph.markdown
L 
L ```console
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 10 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L rails generate devise:views -v registrations confirmations
T 58 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Configuring controllers
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 27 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If the customization at the views level is not enough, you can customize each controller by following these steps:
T 114 text.html.markdown meta.paragraph.markdown
L 
L 1. Create your custom controllers using the generator which requires a scope:
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 77 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L     ```console
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 14 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown fenced_code.block.language
L     rails generate devise:controllers [scope]
T 4 text.html.markdown markup.list.numbered.markdown
T 45 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown
L     ```
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L     If you specify `users` as the scope, controllers will be created in `app/controllers/users/`.
T 4 text.html.markdown markup.list.numbered.markdown
T 19 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 20 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 26 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 73 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 95 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 96 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 97 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L     And the sessions controller will look like this:
T 4 text.html.markdown markup.list.numbered.markdown
T 52 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L     ```ruby
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 11 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L     class Users::SessionsController < Devise::SessionsController
T 4 text.html.markdown markup.list.numbered.markdown
T 9 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby keyword.control.class.ruby
T 10 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 15 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 17 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby punctuation.separator.namespace.ruby
T 35 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 36 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 37 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby punctuation.separator.inheritance.ruby
T 38 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 44 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
T 46 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby punctuation.separator.namespace.ruby
T 64 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
L       # GET /resource/sign_in
T 4 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 29 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L       # def new
T 4 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 15 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L       #   super
T 4 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 15 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L       # end
T 4 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 11 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L       ...
T 4 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 8 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 9 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
L     end
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L     ```
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L     Use the `-c` flag to specify one or more controllers, for example: `rails generate devise:controllers users -c sessions`
T 4 text.html.markdown markup.list.numbered.markdown
T 12 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 13 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 71 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 72 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 123 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 124 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L 2. Tell the router to use this controller:
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 42 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L     ```ruby
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 11 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L     devise_for :users, controllers: { sessions: 'users/sessions' }
T 4 text.html.markdown markup.list.numbered.markdown
T 15 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 16 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 21 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 22 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 23 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 34 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 35 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 36 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 37 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.scope.begin.ruby
T 38 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.syntax.ruby.start-block
T 46 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 47 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 48 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 49 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 63 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 64 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 65 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 66 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.scope.end.ruby
L     ```
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L 3. Recommended but not required: copy (or move) the views from `devise/sessions` to `users/sessions`. Rails will continue using the views from `devise/sessions` due to inheritance if you skip this step, but having the views matching the controller(s) keeps things consistent.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 63 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 64 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 80 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 84 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 85 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 99 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 100 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 143 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 144 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 159 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 160 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 275 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L 4. Finally, change or extend the desired controller actions.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 60 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L     You can completely override a controller action:
T 4 text.html.markdown markup.list.numbered.markdown
T 52 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L     ```ruby
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 11 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L     class Users::SessionsController < Devise::SessionsController
T 4 text.html.markdown markup.list.numbered.markdown
T 9 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby keyword.control.class.ruby
T 10 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 15 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 17 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby punctuation.separator.namespace.ruby
T 35 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 36 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 37 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby punctuation.separator.inheritance.ruby
T 38 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 44 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
T 46 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby punctuation.separator.namespace.ruby
T 64 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
L       def create
T 4 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby keyword.control.def.ruby
T 10 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby
T 16 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby entity.name.function.ruby
L         # custom sign-in code
T 4 text.html.markdown markup.list.numbered.markdown
T 8 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 29 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L       end
T 4 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L     end
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L     ```
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L     Or you can simply add new behavior to it:
T 4 text.html.markdown markup.list.numbered.markdown
T 45 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L     ```ruby
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 11 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L     class Users::SessionsController < Devise::SessionsController
T 4 text.html.markdown markup.list.numbered.markdown
T 9 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby keyword.control.class.ruby
T 10 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 15 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 17 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby punctuation.separator.namespace.ruby
T 35 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 36 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 37 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby punctuation.separator.inheritance.ruby
T 38 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 44 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
T 46 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby punctuation.separator.namespace.ruby
T 64 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
L       def create
T 4 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby keyword.control.def.ruby
T 10 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby
T 16 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.without-arguments.ruby entity.name.function.ruby
L         super do |resource|
T 4 text.html.markdown markup.list.numbered.markdown
T 8 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 13 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.pseudo-method.ruby
T 14 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 16 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.start-block.ruby
T 17 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 18 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
T 26 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby variable.other.block.ruby
T 27 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
L           BackgroundWorker.trigger(resource)
T 4 text.html.markdown markup.list.numbered.markdown
T 10 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 26 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 27 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 34 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 35 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 43 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 44 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
L         end
T 4 text.html.markdown markup.list.numbered.markdown
T 8 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 11 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L       end
T 4 text.html.markdown markup.list.numbered.markdown
T 6 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L     end
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L     ```
T 4 text.html.markdown markup.list.numbered.markdown
T 7 text.html.markdown markup.list.numbered.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L     This is useful for triggering background jobs or logging events during certain actions.
T 4 text.html.markdown markup.list.numbered.markdown
T 91 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L Remember that Devise uses flash messages to let users know if sign in was successful or unsuccessful. Devise expects your application to call `flash[:notice]` and `flash[:alert]` as appropriate. Do not print the entire flash hash, print only specific keys. In some circumstances, Devise adds a `:timedout` key to the flash hash, which is not meant for display. Remove this key from the hash if you intend to print the entire hash.
T 142 text.html.markdown meta.paragraph.markdown
T 143 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 157 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 158 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 163 text.html.markdown meta.paragraph.markdown
T 164 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 177 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 178 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 294 text.html.markdown meta.paragraph.markdown
T 295 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 304 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 305 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 430 text.html.markdown meta.paragraph.markdown
L 
L ### Configuring routes
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 22 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Devise also ships with default routes. If you need to customize them, you should probably be able to do it through the devise_for method. It accepts several options like :class_name, :path_prefix and so on, including the possibility to change path names for I18n:
T 263 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L devise_for :users, path: 'auth', path_names: { sign_in: 'login', sign_out: 'logout', password: 'secret', confirmation: 'verification', unlock: 'unblock', registration: 'register', sign_up: 'cmon_let_me_in' }
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.scope.begin.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.syntax.ruby.start-block
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 83 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 84 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 85 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 93 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 94 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 95 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 96 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 102 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 103 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 104 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 105 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 117 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 118 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 119 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 120 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 132 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 133 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 134 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 135 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 141 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 142 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 143 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 144 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 151 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 152 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 153 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 154 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 166 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 167 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 168 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 169 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 177 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 178 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 179 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 180 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 187 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 188 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 189 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 190 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 204 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 205 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 206 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 207 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.scope.end.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Be sure to check `devise_for` [documentation](https://www.rubydoc.info/gems/devise/ActionDispatch/Routing/Mapper#devise_for-instance_method) for details.
T 17 text.html.markdown meta.paragraph.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 28 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 30 text.html.markdown meta.paragraph.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 139 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 140 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 153 text.html.markdown meta.paragraph.markdown
L 
L If you have the need for more deep customization, for instance to also allow "/sign_in" besides "/users/sign_in", all you need to do is create your routes normally and wrap them in a `devise_scope` block in the router:
T 183 text.html.markdown meta.paragraph.markdown
T 184 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 196 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 197 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 218 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L devise_scope :user do
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.start-block.ruby
L   get 'sign_in', to: 'devise/sessions#new'
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L This way, you tell Devise to use the scope `:user` when "/sign_in" is accessed. Notice `devise_scope` is also aliased as `as` in your router.
T 43 text.html.markdown meta.paragraph.markdown
T 44 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 50 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 87 text.html.markdown meta.paragraph.markdown
T 88 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 100 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 101 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 121 text.html.markdown meta.paragraph.markdown
T 122 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 124 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 125 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 141 text.html.markdown meta.paragraph.markdown
L 
L Please note: You will still need to add `devise_for` in your routes in order to use helper methods such as `current_user`.
T 40 text.html.markdown meta.paragraph.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 107 text.html.markdown meta.paragraph.markdown
T 108 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 120 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 121 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 122 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L devise_for :users, skip: :all
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Hotwire/Turbo
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 17 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Devise integrates with Hotwire/Turbo by treating such requests as navigational, and configuring certain responses for errors and redirects to match the expected behavior. New apps are generated with the following response configuration by default, and existing apps may opt-in by adding the config to their Devise initializers:
T 327 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L Devise.setup do |config|
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.start-block.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby variable.other.block.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
L   # ...
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.whitespace.comment.leading.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L   # When using Devise with Hotwire/Turbo, the http status for error responses
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.whitespace.comment.leading.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L   # and some redirects must match the following. The default in Devise for existing
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.whitespace.comment.leading.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 83 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L   # apps is `200 OK` and `302 Found` respectively, but new apps are generated with
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.whitespace.comment.leading.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L   # these new defaults that match Hotwire/Turbo behavior.
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.whitespace.comment.leading.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L   # Note: These might become the new default in future versions of Devise.
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.whitespace.comment.leading.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L   config.responder.error_status = :unprocessable_content # for Rack 3.1 or higher
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.operator.assignment.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L   # config.responder.error_status = :unprocessable_entity # for Rack 3.0 or lower
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.whitespace.comment.leading.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L   config.responder.redirect_status = :see_other
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.operator.assignment.ruby
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L **Important**: these custom responses require the `responders` gem version to be `3.1.0` or higher, please make sure you update it if you're going to use this configuration. Check [this upgrade guide](https://github.com/heartcombo/devise/wiki/How-To:-Upgrade-to-Devise-4.9.0-[Hotwire-Turbo-integration]) for more info.
T 2 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 13 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 50 text.html.markdown meta.paragraph.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 81 text.html.markdown meta.paragraph.markdown
T 82 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 87 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 88 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 180 text.html.markdown meta.paragraph.markdown
T 181 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 199 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 200 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 201 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 302 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 303 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 318 text.html.markdown meta.paragraph.markdown
L 
L _Note_: the above statuses configuration may become the default for Devise in a future release.
T 1 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 95 text.html.markdown meta.paragraph.markdown
L 
L There are a couple other changes you might need to make in your app to work with Hotwire/Turbo, if you're migrating from rails-ujs:
T 131 text.html.markdown meta.paragraph.markdown
L 
L * The `data-confirm` option that adds a confirmation modal to buttons/forms before submission needs to change to `data-turbo-confirm`, so that Turbo handles those appropriately.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 113 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 114 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 132 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 133 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 177 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * The `data-method` option that sets the request method for link submissions needs to change to `data-turbo-method`. This is not necessary for `button_to` or `form`s since Turbo can handle those.
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 7 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 96 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 97 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 114 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 115 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 143 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 144 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 153 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 154 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 158 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 159 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 163 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 164 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 195 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L If you're setting up Devise to sign out via `:delete`, and you're using links (instead of buttons wrapped in a form) to sign out with the `method: :delete` option, they will need to be updated as described above. (Devise does not provide sign out links/buttons in its shared views.)
T 44 text.html.markdown meta.paragraph.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 138 text.html.markdown meta.paragraph.markdown
T 139 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 154 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 155 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 282 text.html.markdown meta.paragraph.markdown
L 
L Make sure to inspect your views looking for those, and change appropriately.
T 76 text.html.markdown meta.paragraph.markdown
L 
L ### I18n
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 8 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Devise uses flash messages with I18n, in conjunction with the flash keys :notice and :alert. To customize your app, you can set up your locale file:
T 148 text.html.markdown meta.paragraph.markdown
L 
L ```yaml
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L en:
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L   devise:
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L     sessions:
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L       signed_in: 'Signed in successfully.'
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.begin.yaml
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.end.yaml
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can also create distinct messages based on the resource you've configured using the singular name given in routes:
T 118 text.html.markdown meta.paragraph.markdown
L 
L ```yaml
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L en:
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L   devise:
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L     sessions:
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L       user:
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L         signed_in: 'Welcome user, you are signed in.'
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.begin.yaml
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.end.yaml
L       admin:
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L         signed_in: 'Hello admin!'
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.begin.yaml
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.end.yaml
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The Devise mailer uses a similar pattern to create subject messages:
T 68 text.html.markdown meta.paragraph.markdown
L 
L ```yaml
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L en:
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L   devise:
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L     mailer:
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L       confirmation_instructions:
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L         subject: 'Hello everybody!'
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.begin.yaml
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.end.yaml
L         user_subject: 'Hello User! Please confirm your email'
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.begin.yaml
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.end.yaml
L       reset_password_instructions:
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
L         subject: 'Reset instructions'
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.unquoted.plain.out.yaml entity.name.tag.yaml
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml punctuation.separator.key-value.mapping.yaml
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.begin.yaml
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.yaml string.quoted.single.yaml punctuation.definition.string.end.yaml
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Take a look at our locale file to check all available messages. You may also be interested in one of the many translations that are available on our wiki:
T 154 text.html.markdown meta.paragraph.markdown
L 
L https://github.com/heartcombo/devise/wiki/I18n
T 46 text.html.markdown meta.paragraph.markdown
L 
L Caution: Devise Controllers inherit from ApplicationController. If your app uses multiple locales, you should be sure to set I18n.locale in ApplicationController.
T 162 text.html.markdown meta.paragraph.markdown
L 
L ### Test helpers
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 16 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Devise includes some test helpers for controller and integration tests.
T 71 text.html.markdown meta.paragraph.markdown
L In order to use them, you need to include the respective module in your test
T 76 text.html.markdown meta.paragraph.markdown
L cases/specs.
T 12 text.html.markdown meta.paragraph.markdown
L 
L ### Controller tests
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 20 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Controller tests require that you include `Devise::Test::IntegrationHelpers` on
T 42 text.html.markdown meta.paragraph.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 76 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown
L your test case or its parent `ActionController::TestCase` superclass.
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 69 text.html.markdown meta.paragraph.markdown
L For Rails versions prior to 5, include `Devise::Test::ControllerHelpers` instead, since the superclass
T 39 text.html.markdown meta.paragraph.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 71 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 72 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 102 text.html.markdown meta.paragraph.markdown
L for controller tests was changed to ActionDispatch::IntegrationTest
T 67 text.html.markdown meta.paragraph.markdown
L (for more details, see the [Integration tests](#integration-tests) section).
T 27 text.html.markdown meta.paragraph.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 66 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 76 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L class PostsControllerTest < ActionController::TestCase
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby keyword.control.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby punctuation.separator.inheritance.ruby
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby punctuation.separator.namespace.ruby
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
L   include Devise::Test::IntegrationHelpers # Rails >= 5
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.other.special-method.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L class PostsControllerTest < ActionController::TestCase
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby keyword.control.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby punctuation.separator.inheritance.ruby
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby punctuation.separator.namespace.ruby
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
L   include Devise::Test::ControllerHelpers # Rails < 5
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.other.special-method.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L If you're using RSpec, you can put the following inside a file named
T 68 text.html.markdown meta.paragraph.markdown
L `spec/support/devise.rb` or in your `spec/spec_helper.rb` (or
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown meta.paragraph.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown meta.paragraph.markdown
L `spec/rails_helper.rb` if you are using `rspec-rails`):
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 40 text.html.markdown meta.paragraph.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L RSpec.configure do |config|
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.start-block.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby variable.other.block.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
L   config.include Devise::Test::ControllerHelpers, type: :controller
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.other.special-method.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L   config.include Devise::Test::ControllerHelpers, type: :view
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.other.special-method.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Just be sure that this inclusion is made *after* the `require 'rspec/rails'` directive.
T 41 text.html.markdown meta.paragraph.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 53 text.html.markdown meta.paragraph.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 76 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 87 text.html.markdown meta.paragraph.markdown
L 
L Now you are ready to use the `sign_in` and `sign_out` methods on your controller
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 38 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 43 text.html.markdown meta.paragraph.markdown
T 44 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown meta.paragraph.markdown
L tests:
T 6 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L sign_in @user
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.readwrite.instance.ruby punctuation.definition.variable.ruby
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.readwrite.instance.ruby
L sign_in @user, scope: :admin
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.readwrite.instance.ruby punctuation.definition.variable.ruby
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.readwrite.instance.ruby
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L If you are testing Devise internal controllers or a controller that inherits
T 76 text.html.markdown meta.paragraph.markdown
L from Devise's, you need to tell Devise which mapping should be used before a
T 76 text.html.markdown meta.paragraph.markdown
L request. This is necessary because Devise gets this information from the router,
T 80 text.html.markdown meta.paragraph.markdown
L but since controller tests do not pass through the router, it needs to be stated
T 80 text.html.markdown meta.paragraph.markdown
L explicitly. For example, if you are testing the user scope, simply use:
T 71 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L test 'GET new' do
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.function.kernel.ruby
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.start-block.ruby
L   # Mimic the router behavior of setting the Devise scope through the env.
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.whitespace.comment.leading.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L   @request.env['devise.mapping'] = Devise.mappings[:user]
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.readwrite.instance.ruby punctuation.definition.variable.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.readwrite.instance.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.begin.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.end.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.operator.assignment.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.begin.ruby
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.end.ruby
L 
L   # Use the sign_in helper to sign in a fixture `User` record.
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.whitespace.comment.leading.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L   sign_in users(:alice)
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.section.function.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.section.function.ruby
L 
L   get :new
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L 
L   # assert something
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.whitespace.comment.leading.ruby
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Integration tests
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 21 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Integration test helpers are available by including the
T 55 text.html.markdown meta.paragraph.markdown
L `Devise::Test::IntegrationHelpers` module.
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L class PostsTests < ActionDispatch::IntegrationTest
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby keyword.control.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.name.type.class.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby punctuation.separator.inheritance.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby punctuation.separator.namespace.ruby
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.class.ruby entity.other.inherited-class.ruby
L   include Devise::Test::IntegrationHelpers
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.other.special-method.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Now you can use the following `sign_in` and `sign_out` methods in your integration
T 30 text.html.markdown meta.paragraph.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 38 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 39 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown meta.paragraph.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 82 text.html.markdown meta.paragraph.markdown
L tests:
T 6 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L sign_in users(:bob)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.section.function.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.section.function.ruby
L sign_in users(:bob), scope: :admin
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.section.function.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby constant.language.symbol.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby punctuation.section.function.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L 
L sign_out :user
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L RSpec users can include the `IntegrationHelpers` module on their `:feature` specs.
T 28 text.html.markdown meta.paragraph.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown meta.paragraph.markdown
T 66 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 74 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 75 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 82 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L RSpec.configure do |config|
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.start-block.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby variable.other.block.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
L   config.include Devise::Test::IntegrationHelpers, type: :feature
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.other.special-method.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Unlike controller tests, integration tests do not need to supply the
T 68 text.html.markdown meta.paragraph.markdown
L `devise.mapping` `env` value, as the mapping can be inferred by the routes that
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown meta.paragraph.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown
L are executed in your tests.
T 27 text.html.markdown meta.paragraph.markdown
L 
L You can read more about testing your Rails controllers with RSpec in the wiki:
T 78 text.html.markdown meta.paragraph.markdown
L 
L * https://github.com/heartcombo/devise/wiki/How-To:-Test-controllers-with-Rails-(and-RSpec)
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 91 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ### OmniAuth
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 12 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Devise comes with OmniAuth support out of the box to authenticate with other providers. To use it, simply specify your OmniAuth configuration in `config/initializers/devise.rb`:
T 145 text.html.markdown meta.paragraph.markdown
T 146 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 175 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 176 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 177 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L config.omniauth :github, 'APP_ID', 'APP_SECRET', scope: 'user,public_repo'
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.begin.ruby
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby string.quoted.single.ruby punctuation.definition.string.end.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can read more about OmniAuth support in the wiki:
T 53 text.html.markdown meta.paragraph.markdown
L 
L * https://github.com/heartcombo/devise/wiki/OmniAuth:-Overview
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ### Configuring multiple models
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 31 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Devise allows you to set up as many Devise models as you want. If you want to have an Admin model with just authentication and timeout features, in addition to the User model above, just run:
T 191 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L # Create a migration with the required fields
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L create_table :admins do |t|
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.start-block.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby variable.other.block.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.block.parameters.ruby punctuation.separator.variable.ruby
L   t.string :email
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L   t.string :encrypted_password
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L   t.timestamps null: false
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.hashkey.ruby punctuation.definition.constant.hashkey.ruby
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.boolean.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L 
L # Inside your Admin model
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L devise :database_authenticatable, :timeoutable
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L 
L # Inside your routes
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L devise_for :admins
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L 
L # Inside your protected controller
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L before_action :authenticate_admin!
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L 
L # Inside your controllers and views
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby punctuation.definition.comment.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby comment.line.number-sign.ruby
L admin_signed_in?
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
L current_admin
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
L admin_session
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Alternatively, you can simply run the Devise generator.
T 55 text.html.markdown meta.paragraph.markdown
L 
L Keep in mind that those models will have completely different routes. They **do not** and **cannot** share the same controller for sign in, sign out and so on. In case you want to have different roles sharing the same actions, we recommend that you use a role-based approach, by either providing a role column or using a dedicated gem for authorization.
T 75 text.html.markdown meta.paragraph.markdown
T 77 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 83 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 85 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 90 text.html.markdown meta.paragraph.markdown
T 92 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 98 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 100 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 353 text.html.markdown meta.paragraph.markdown
L 
L ### Active Job Integration
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 26 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If you are using Active Job to deliver Action Mailer messages in the
T 68 text.html.markdown meta.paragraph.markdown
L background through a queuing back-end, you can send Devise emails through your
T 78 text.html.markdown meta.paragraph.markdown
L existing queue by overriding the `send_devise_notification` method in your model.
T 33 text.html.markdown meta.paragraph.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 81 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L def send_devise_notification(notification, *args)
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby keyword.control.def.ruby
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby entity.name.function.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby punctuation.definition.parameters.ruby
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby variable.parameter.function.ruby
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby punctuation.separator.object.ruby
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby storage.type.variable.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby variable.parameter.function.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function.method.with-arguments.ruby punctuation.definition.parameters.ruby
L   devise_mailer.send(notification, self, *args).deliver_later
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.language.self.ruby
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.operator.arithmetic.ruby
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.function.ruby
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
L end
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.control.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Password reset tokens and Rails logs
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 40 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L If you enable the [Recoverable](https://www.rubydoc.info/gems/devise/Devise/Models/Recoverable) module, note that a stolen password reset token could give an attacker access to your application. Devise takes effort to generate random, secure tokens, and stores only token digests in the database, never plaintext. However the default logging behavior in Rails can cause plaintext tokens to leak into log files:
T 18 text.html.markdown meta.paragraph.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 94 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 95 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 410 text.html.markdown meta.paragraph.markdown
L 
L 1. Action Mailer logs the entire contents of all outgoing emails to the DEBUG level. Password reset tokens delivered to users in email will be leaked.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 150 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 2. Active Job logs all arguments to every enqueued job at the INFO level. If you configure Devise to use `deliver_later` to send password reset emails, password reset tokens will be leaked.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 105 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
T 106 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 119 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 120 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 189 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L Rails sets the production logger level to INFO by default. Consider changing your production logger level to WARN if you wish to prevent tokens from being leaked into your logs. In `config/environments/production.rb`:
T 181 text.html.markdown meta.paragraph.markdown
T 182 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 215 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 216 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 217 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L config.log_level = :warn
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.operator.assignment.ruby
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L 
L ### Other ORMs
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 14 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Devise supports ActiveRecord (default) and Mongoid. To select another ORM, simply require it in the initializer file.
T 117 text.html.markdown meta.paragraph.markdown
L 
L ### Rails API Mode
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 18 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Rails 5+ has a built-in [API Mode](https://edgeguides.rubyonrails.org/api_app.html) which optimizes Rails for use as an API (only). Devise is _somewhat_ able to handle applications that are built in this mode without additional modifications in the sense that it should not raise exceptions and the like. But some issues may still arise during `development`/`testing`, as we still don't know the full extent of this compatibility. (For more information, see [issue #4947](https://github.com/heartcombo/devise/issues/4947/))
T 24 text.html.markdown meta.paragraph.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 82 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 83 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 142 text.html.markdown meta.paragraph.markdown
T 143 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 151 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 152 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 344 text.html.markdown meta.paragraph.markdown
T 345 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 356 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 357 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 358 text.html.markdown meta.paragraph.markdown
T 359 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 366 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 367 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 458 text.html.markdown meta.paragraph.markdown
T 459 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 470 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 471 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 472 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 521 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 522 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 523 text.html.markdown meta.paragraph.markdown
L 
L #### Supported Authentication Strategies
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 40 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L API-only applications don't support browser-based authentication via cookies, which is devise's default. Yet, devise can still provide authentication out of the box in those cases with the `http_authenticatable` strategy, which uses HTTP Basic Auth and authenticates the user on each request. (For more info, see this wiki article for [How To: Use HTTP Basic Authentication](https://github.com/heartcombo/devise/wiki/How-To:-Use-HTTP-Basic-Authentication))
T 189 text.html.markdown meta.paragraph.markdown
T 190 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 210 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 211 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 335 text.html.markdown meta.paragraph.markdown
T 336 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 373 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 374 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 375 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 454 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 455 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 456 text.html.markdown meta.paragraph.markdown
L 
L The devise default for HTTP Auth is disabled, so it will need to be enabled in the devise initializer for the database strategy:
T 128 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L config.http_authenticatable = [:database]
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby keyword.operator.assignment.ruby
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.begin.ruby
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby punctuation.definition.constant.ruby
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby constant.language.symbol.ruby
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.section.array.end.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L This restriction does not limit you from implementing custom warden strategies, either in your application or via gem-based extensions for devise.
T 146 text.html.markdown meta.paragraph.markdown
L A common authentication strategy for APIs is token-based authentication. For more information on extending devise to support this type of authentication and others, see the wiki article for [Simple Token Authentication Examples and alternatives](https://github.com/heartcombo/devise/wiki/How-To:-Simple-Token-Authentication-Example#alternatives) or this blog post on [Custom authentication methods with Devise](https://blog.plataformatec.com.br/2019/01/custom-authentication-methods-with-devise/).
T 190 text.html.markdown meta.paragraph.markdown
T 191 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 244 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 245 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 246 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 344 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 345 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 367 text.html.markdown meta.paragraph.markdown
T 368 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 409 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 410 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 411 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 495 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 496 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 497 text.html.markdown meta.paragraph.markdown
L 
L #### Testing
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 12 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L API Mode changes the order of the middleware stack, and this can cause problems for `Devise::Test::IntegrationHelpers`. This problem usually surfaces as an ```undefined method `[]=' for nil:NilClass``` error when using integration test helpers, such as `#sign_in`. The solution is simply to reorder the middlewares by adding the following to test.rb:
T 84 text.html.markdown meta.paragraph.markdown
T 85 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 117 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 118 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 156 text.html.markdown meta.paragraph.markdown
T 159 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 198 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 201 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 253 text.html.markdown meta.paragraph.markdown
T 254 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 262 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 263 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 350 text.html.markdown meta.paragraph.markdown
L 
L ```ruby
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L Rails.application.config.middleware.insert_before Warden::Manager, ActionDispatch::Cookies
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 83 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 90 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
L Rails.application.config.middleware.insert_before Warden::Manager, ActionDispatch::Session::CookieStore
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.method.ruby
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby meta.function-call.ruby entity.name.function.ruby
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.object.ruby
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 83 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 90 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby support.class.ruby
T 92 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby punctuation.separator.namespace.ruby
T 103 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.ruby variable.other.constant.ruby
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L For a deeper understanding of this, review [this issue](https://github.com/heartcombo/devise/issues/4696).
T 43 text.html.markdown meta.paragraph.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 104 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 105 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 106 text.html.markdown meta.paragraph.markdown
L 
L Additionally be mindful that without views supported, some email-based flows from Confirmable, Recoverable and Lockable are not supported directly at this time.
T 160 text.html.markdown meta.paragraph.markdown
L 
L ## Additional information
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 25 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L ### Warden
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 10 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Devise is based on Warden, which is a general Rack authentication framework created by Daniel Neighman. We encourage you to read more about Warden here:
T 152 text.html.markdown meta.paragraph.markdown
L 
L https://github.com/wardencommunity/warden
T 41 text.html.markdown meta.paragraph.markdown
L 
L ## License
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 10 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L MIT License.
T 12 text.html.markdown meta.paragraph.markdown
L Copyright 2020-CURRENT Rafael França, Carlos Antonio da Silva.
T 63 text.html.markdown meta.paragraph.markdown
L Copyright 2009-2019 Plataformatec.
T 34 text.html.markdown meta.paragraph.markdown
L 
L The Devise logo is licensed under [Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License](https://creativecommons.org/licenses/by-nc-nd/4.0/).
T 34 text.html.markdown meta.paragraph.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 117 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 118 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 119 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 169 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 170 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 171 text.html.markdown meta.paragraph.markdown
L 
