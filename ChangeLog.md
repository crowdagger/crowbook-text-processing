ChangeLog
==========

unreleased
------------
Breaking changes as the API was modified:
* The module `french` is no longer public, only `FrenchFormatter` is.
* `typographic_quotes` has been renamed `quotes` and is no longer
  directly exported, use `clean::quotes`.
* `remove_whitespaces` has been renamed `whitespaces` and is no longer
  directly exported, use `clean::whitespaces`.
* `ellipsis` is no longer directly exported, use `clean::ellipsis`.
* `escape_html` has been renamed `html` and is no longer directly
  exported, use `escape::html`.
* `escape_tex` has been renamed `tex` and is no longer directly
  exported, use `escape::tex`.
* `escape_nb_spaces` has been renamed `nb_spaces` and is no longer directly
  exported, use `escape::nb_spaces`.
* `escape_nb_spaces_tex` has been renamed `nb_spaces_tex` and is no longer directly
  exported, use `escape::nb_spaces_tex`.
* `escape_quotes` has been renamed `quotes` and is no longer directly
  exported, use `escape::quotes`.
  

0.1.6 (2016-10-19)
----------------------
* Enhanced `typographic_quotes`'s heuristics, and added more tests.
* Added `ellpisis` function, and use it for in `FrenchFormatter`.

0.1.5 (2016-10-18)
----------------------
* Enhanced `typographic_quotes`'s heuristics.

0.1.4 (2016-10-17)
---------------------
* Added `typographic_quotes` function.
* Made`FrenchFormatter` use it by default (can be disabled by setting 
  `typographic_quotes` to false).

0.1.3 (2016-10-13)
----------------------
* Now use Travis for continuous integration.
* Found & documented rustc minimal version to build this lib (1.6.0)
* `FrenchFormatter` now implements the `Default` and `Debug` traits.
* Some functions or structs are now reexported so they can be accessed
  more easily: 
    * `FrenchFormatter`,
    * `escape_html`,
	* `escape_tex`,
	* `remove_whitespaces`.
	

0.1.2 (2016-10-12)
----------------------
* Added `format_tex` to the `FrenchFormatter`.

	
0.1.1 (2016-10-06)
--------------------
* Added `remove_whitespaces` from Crowbook 0.9.1.
* Added `FrenchFormatter` which is basically the `French` cleaner from
  Crowbook 0.9.1.
* Added `escape_nb_spaces_tex`.

0.1.0 (2016-10-06)
--------------------
* Initial release, taking `escape_html`, `escape_tex`,
  `escape_nb_spaces` and `escape_quotes` from Crowbook 0.9.1.
