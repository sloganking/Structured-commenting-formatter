## FAQ

- [Why a third party tool?](#why_third_party)
- [What if I need to ensure ``max_width``?](#max_width)

<h3 name="why_third_party">
Why a third party tool?
</h3>

Convincing all existing code formatter devs to support structured commenting, and maintaining support for all of those implementations would be difficult. Building one tool that supports multiple languages is much easier, and doesn't burden other developers with the technical debt of maintinaing support.

<h3 name="max_width">
What if I need to ensure max_width?
</h3>

While there's nothing that makes structured commenting incompatible with ensuring a ``max_width``, ensuring a ``max_width`` by spliting up long lines of code into multiple shorter ones, is really the job of traditional code formatters. If you love structured comments, but must have a ``max_width``, consider asking for structured comment formatting support to be added to your code formatter of choice.
