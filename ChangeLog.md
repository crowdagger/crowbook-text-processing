ChangeLog
==========

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
