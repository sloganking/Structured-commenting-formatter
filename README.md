# Structured commenting formatter
 
Since using comment brackets ``//>``,  ``//<`` and ``//<>`` would allow [structured comments](https://github.com/sloganking/Structured-Commenting) to be understood, and recovered, even after whitespace and format mangling. An "unmangle the structured comments" tool could be created and run directly after traditional code formatters. Meaning we could get the desired format, for almost all languages, without modifying existing code formatters designed for those languages. This makes

- programming a structured commenting formatter extremely simple
- Only have to program 1 formatter, that would work with all languages.
- Not burden existing code formatter devs with implementing and maintaining support for structured comments.

The only downside, would be additional computation running this as a post-traditional-formatter cycle. As baking support for structured commenting into traditional code formatters would be less resource intensive. Although extremely developer and maintenance heavy. So the tradeoff is worth it.
