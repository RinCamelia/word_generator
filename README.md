# Conlanger's Word Generator

A while back, there was an online word generator at... honestly, I don't remember where it was. Anyway, it was relatively full featured but the site got infected with malware and was no longer usable. Zompist has a word generator that isn't bad but I found the mechanics and features personally confusing, so I set out to replicate the general functionality and structure of the other tool as I remembered it.

Run word_generator.exe to produce a file with randomly generated words conforming to the configuration described below. Not too much else to it, no current plans to do other tasks like track existing vocabulary or other conlang related tasks (though I do want to give a shoutout to [PolyGlot](http://sulmere.tumblr.com/PolyGlot) for vocabulary management). It's best used to generate into IPA or orthographies for your language, unless your writing system uses a character set that exists in Unicode.

# Config

The config file is written in JSON for ease of implementation. There are several [resources](http://www.w3schools.com/json/) and [linters](http://jsonlint.com/) for JSON syntax; however, if you come across valid/linted JSON that is getting some sort of parse error, please report it to me.

* **output_settings:** Settings related to formatting and output.
  * **show_word_rewrites:** Outputs the chain of applied grapheme rewrite rules for each word that has been rewritten. Also shows syllable rewrites if show_syllable_strings is enabled.
  * **show_syllable_strings:** Writes each word's final syllable string in parentheses next to the word.
  * **only_mark_rejects:** Instead of omitting rejected words, output them and note which reject rules rejected them.
  * **output_file:** the name of the file to output words into. **Note**: This file will be silently overwritten if it already exists. Save the files with words you like!
  * **word_count:** The number of words to generate per run. Note that this is after rejections- so if only_mark_rejects is on then you will probably get more.
* generate_settings: Settings related to word generation itself
  * **syllable_decay_rate:** Each word must have at least one syllable; the chance for words with syllable length N > 1 is *(1 - syllable_decay_rate) x (N - 1)*.
  * **max_syllables:** Hard cap on the maximum syllable count for a single word.
  * **rewrites_before_rejects:** If true, apply all rewrites then mark any reject rules. If false, mark word rejects then do all rewriting.
* **graphemes:** list of groups of graphemes in the following format:
  * **name:** Name for the grapheme group. Needs to be one grapheme.
    * It's easiest to use the roman alphabet; other characters may work but are not guaranteed to behave as expected, due to the fact that rust has no simple conception of a character and I'm relying on a library's implementation of the unicode definition of a grapheme.
 * **graphemes:** A list of the graphemes that can be generated for this group, in the following format:
   * **string:** The string inserted for this specific grapheme. This does not need to be one character long and can be any UTF-8 string of code points.
   * **weight:** How likely the generator is to pick this grapheme for this group. The likelihood for each grapheme's selection is *weight / (sum of weights in this grapheme group)*.
* **syllables:** A list of syllable definitions that can be generated, in the following format:
  * **string:** A string of several characters, composed of names of grapheme groups.
  * **weight:** How likely the generator is to pick this syllable. The likelihood for each syllable's selection is *weight / (sum of assigned syllable weights)*.
  * **only_first_syllable:** If true, this syllable will be constrained to only being generated for the first syllable for a word. Is not exclusive with only_last_syllable (i.e. both on = generates for first or last and not other syllables)
  * **only_last_syllable:** If true, this syllable will be constrained to only being generated for the last syllable in a word. Is not exclusive with only_first_syllable (i.e. both on = generates for first or last and not other syllables)
* **rewrites:**
  * **syllable_rewrites:** A list of all syllable level rewrite rules in the following format:
    * **pattern:** A regex describing the rewrite rule. All non-overlapping matching instances will be replaced.
    * **replace:** A string of syllables to replace the pattern matches with. This can be more than one character but has to be all valid grapheme groups.
  * **grapheme_rewrites:** A list of all grapheme level rewrite rules. Similar to syllable_rewrites, except the replacement string does NOT have to be a grapheme already listed in a grapheme group.
* **rejects:**
  * **syllable_rejects**: a list of regexes that will be checked against each word's syllable string. If any match the word will be rejected.
  * **grapheme rejects:** a list of graphemes that will be checked against each word's grapheme string. If any match the word will be rejected.

For more info about the specific flavor of regex supported, see these [two](http://doc.rust-lang.org/regex/regex/index.html) [resources]{https://re2.googlecode.com/hg/doc/syntax.html}. [This link](https://regex-golang.appspot.com/assets/html/index.html) has a regex tester conforming to that syntax reference in case you need one.

# Mac or Linux/Building

I don't have mac or linux environments on hand, but you can compile the project from source yourself easily enough with Rust and the source. Download the source either from a release or by using git to clone the repository, then:

> Download and install [Rust](http://www.rust-lang.org/) for your platform  
> Open the source folder (the one with Cargo.toml in it) in a terminal and type 'cargo build'  
> There should now be a word_generator program in the /target/debug folder

If you run into any issues doing so, please report them so I can take a look.

# Todo

* Allow command line input of config file names to ease use of multiple configs if desired
* Clean up syllable definitions in config and come up with a better solution than the first or last only generation (probably involving rejects)
* Improve formatting of output, particularly with word transforms and syllable strings enabled
