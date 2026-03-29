use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

#[derive(Clone, Copy, Debug, Default)]
pub enum OutputFormat {
	#[default]
	Html,
	Godot,
}

pub struct Alm {
	bold: bool,
	italic: bool,
	underline: bool,
	strikethrough: bool,
	spoiler: bool,
}

impl Alm {
	pub fn parse(string: &str, format: OutputFormat) -> Result<String, pest::error::Error<Rule>> {
		let mut inst = Self {
			bold: false,
			italic: false,
			underline: false,
			strikethrough: false,
			spoiler: false,
		};
		let pairs = AlmParser::parse(Rule::document, string)?;
		match format {
			OutputFormat::Html => inst.f(pairs, "Html"),
			OutputFormat::Godot => inst.f(pairs, "Godot"),
		}
	}

	fn f(&mut self, pairs: Pairs<'_, Rule>, x: &str) -> Result<String, pest::error::Error<Rule>> {
		let mut o = String::new();

		for p in pairs.into_iter() {
			// println!("{:?}: \"{}\"", p.as_rule(), p.as_str());
			match p.as_rule() {
				Rule::WHITESPACE
				| Rule::NEWLINE
				| Rule::code_line
				| Rule::link_text
				| Rule::link_url
				| Rule::image_alt
				| Rule::image_url
				| Rule::plain_text => {
					o.push_str(p.as_str());
				}
				Rule::document
				| Rule::newline_separated_blocks_without_the_delimiting_newlines
				| Rule::non_line_block
				| Rule::heading
				| Rule::h1
				| Rule::h2
				| Rule::h3
				| Rule::h4
				| Rule::h5
				| Rule::h6
				| Rule::code_fence
				| Rule::blockquote
				| Rule::blockquote_line
				| Rule::unordered_list
				| Rule::unordered_list_item
				| Rule::unordered_list_marker
				| Rule::ordered_list
				| Rule::ordered_list_item
				| Rule::ordered_list_marker
				| Rule::line
				| Rule::inline
				| Rule::inline_left_word_boundary
				| Rule::inline_no_word_boundary
				| Rule::words
				| Rule::code_span
				| Rule::superscript
				| Rule::subscript
				| Rule::color
				| Rule::font_size
				| Rule::variable_substitution => {
					o.push_str(self.f(p.into_inner(), x)?.as_str());
				}
				Rule::OMITTED_WHITESPACE
				| Rule::OMITTED_NEWLINE
				| Rule::frontmatter
				| Rule::frontmatter_line
				| Rule::code_info
				| Rule::blockquote_marker
				| Rule::EOI => {}
				Rule::bold_on => {
					if self.bold {
						o.push_str(match x {
							"Godot" => "[/b]",
							"Html" => "</b>",
							_ => panic!(),
						});
					} else {
						self.bold = true;
						o.push_str(match x {
							"Godot" => "[b]",
							"Html" => "<b>",
							_ => panic!(),
						});
					}
				}
				Rule::italic_on => {
					if self.italic {
						o.push_str(match x {
							"Godot" => "[/i]",
							"Html" => "</i>",
							_ => panic!(),
						});
					} else {
						self.italic = true;
						o.push_str(match x {
							"Godot" => "[i]",
							"Html" => "<i>",
							_ => panic!(),
						});
					}
				}
				Rule::underline_on => {
					if self.underline {
						o.push_str(match x {
							"Godot" => "[/u]",
							"Html" => "</u>",
							_ => panic!(),
						});
					} else {
						self.underline = true;
						o.push_str(match x {
							"Godot" => "[u]",
							"Html" => "<u>",
							_ => panic!(),
						});
					}
				}
				Rule::strikethrough_on => {
					if self.strikethrough {
						o.push_str(match x {
							"Godot" => "[/s]",
							"Html" => "</s>",
							_ => panic!(),
						});
					} else {
						self.strikethrough = true;
						o.push_str(match x {
							"Godot" => "[s]",
							"Html" => "<s>",
							_ => panic!(),
						});
					}
				}
				Rule::spoiler_on => {
					if self.spoiler {
						o.push_str(match x {
							"Godot" => "[/fgcolor]",
							"Html" => "</details>",
							_ => panic!(),
						});
					} else {
						self.spoiler = true;
						o.push_str(match x {
							"Godot" => "[fgcolor=black]",
							"Html" => "<details><summary>Spoiler</summary>",
							_ => panic!(),
						});
					}
				}
				Rule::bold_off => {
					if self.bold {
						self.bold = false;
						o.push_str(match x {
							"Godot" => "[/b]",
							"Html" => "</b>",
							_ => panic!(),
						});
					} else {
						o.push_str(p.as_str());
					}
				}
				Rule::italic_off => {
					if self.italic {
						self.italic = false;
						o.push_str(match x {
							"Godot" => "[i]",
							"Html" => "<i>",
							_ => panic!(),
						});
					} else {
						o.push_str(p.as_str());
					}
				}
				Rule::underline_off => {
					if self.underline {
						self.underline = false;
						o.push_str(match x {
							"Godot" => "[u]",
							"Html" => "<u>",
							_ => panic!(),
						});
					} else {
						o.push_str(p.as_str());
					}
				}
				Rule::strikethrough_off => {
					if self.strikethrough {
						self.strikethrough = false;
						o.push_str(match x {
							"Godot" => "[s]",
							"Html" => "<s>",
							_ => panic!(),
						});
					} else {
						o.push_str(p.as_str());
					}
				}
				Rule::spoiler_off => {
					if self.spoiler {
						self.spoiler = false;
						o.push_str(match x {
							"Godot" => "[/fgcolor]",
							"Html" => "</details>",
							_ => panic!(),
						});
					} else {
						o.push_str(p.as_str());
					}
				}
				Rule::h1_inner => {
					o.push_str(match x {
						"Godot" => "[font_size=32]",
						"Html" => "<h1>",
						_ => panic!(),
					});
					o.push_str(p.as_str());
					o.push_str(match x {
						"Godot" => "[/font_size]",
						"Html" => "</h1>",
						_ => panic!(),
					});
				}
				Rule::h2_inner => {
					o.push_str(match x {
						"Godot" => "[font_size=28]",
						"Html" => "<h2>",
						_ => panic!(),
					});
					o.push_str(p.as_str());
					o.push_str(match x {
						"Godot" => "[/font_size]",
						"Html" => "</h2>",
						_ => panic!(),
					});
				}
				Rule::h3_inner => {
					o.push_str(match x {
						"Godot" => "[font_size=24]",
						"Html" => "<h3>",
						_ => panic!(),
					});
					o.push_str(p.as_str());
					o.push_str(match x {
						"Godot" => "[/font_size]",
						"Html" => "</h3>",
						_ => panic!(),
					});
				}
				Rule::h4_inner => {
					o.push_str(match x {
						"Godot" => "[font_size=20]",
						"Html" => "<h4>",
						_ => panic!(),
					});
					o.push_str(p.as_str());
					o.push_str(match x {
						"Godot" => "[/font_size]",
						"Html" => "</h4>",
						_ => panic!(),
					});
				}
				Rule::h5_inner => {
					o.push_str(match x {
						"Godot" => "[font_size=16]",
						"Html" => "<h5>",
						_ => panic!(),
					});
					o.push_str(p.as_str());
					o.push_str(match x {
						"Godot" => "[/font_size]",
						"Html" => "</h5>",
						_ => panic!(),
					});
				}
				Rule::h6_inner => {
					o.push_str(match x {
						"Godot" => "[font_size=12]",
						"Html" => "<h6>",
						_ => panic!(),
					});
					o.push_str(p.as_str());
					o.push_str(match x {
						"Godot" => "[/font_size]",
						"Html" => "</h6>",
						_ => panic!(),
					});
				}
				Rule::hr => {
					o.push_str(match x {
						"Godot" => "[hr]",
						"Html" => "<hr />",
						_ => panic!(),
					});
				}
				Rule::fence_open => {
					o.push_str(match x {
						"Godot" => "[code]",
						"Html" => "<code>",
						_ => panic!(),
					});
				}
				Rule::fence_close => {
					o.push_str(match x {
						"Godot" => "[/code]",
						"Html" => "</code>",
						_ => panic!(),
					});
				}
				Rule::code_span_inner => {
					o.push_str(match x {
						"Godot" => "[code]",
						"Html" => "<code>",
						_ => panic!(),
					});
					o.push_str(p.as_str());
					o.push_str(match x {
						"Godot" => "[/code]",
						"Html" => "</code>",
						_ => panic!(),
					});
				}
				Rule::superscript_inner => {
					o.push_str(match x {
						"Godot" => "[sup]",
						"Html" => "<sup>",
						_ => panic!(),
					});
					o.push_str(p.as_str());
					o.push_str(match x {
						"Godot" => "[/sup]",
						"Html" => "</sup>",
						_ => panic!(),
					});
				}
				Rule::subscript_inner => {
					o.push_str(match x {
						"Godot" => "[sub]",
						"Html" => "<sub>",
						_ => panic!(),
					});
					o.push_str(p.as_str());
					o.push_str(match x {
						"Godot" => "[/sub]",
						"Html" => "</sub>",
						_ => panic!(),
					});
				}
				Rule::link => {
					let text_url = parse_link_or_img(p.into_inner());
					o.push_str(match x {
						"Godot" => "[url=",
						"Html" => "<a href=\">",
						_ => panic!(),
					});
					o.push_str(text_url.1.as_str());
					o.push_str(match x {
						"Godot" => "]",
						"Html" => "\">",
						_ => panic!(),
					});
					o.push_str(text_url.0.as_str());
					o.push_str(match x {
						"Godot" => "[/url]",
						"Html" => "</a>",
						_ => panic!(),
					});
				}
				Rule::image => {
					let text_url = parse_link_or_img(p.into_inner());
					o.push_str(match x {
						"Godot" => "[img]",
						"Html" => "<img src=\"",
						_ => panic!(),
					});
					o.push_str(text_url.1.as_str());
					if x == "Html" {
						o.push_str("\" alt=\"");
						o.push_str(text_url.0.as_str());
					}
					o.push_str(match x {
						"Godot" => "[/img]",
						"Html" => "\"/>",
						_ => panic!(),
					});
				}
				Rule::color_code => {
					let str = p.as_str();
					if str.is_empty() {
						o.push_str("[/color]");
					} else {
						o.push_str("[color=");
						o.push_str(str);
						o.push_str("]");
					}
				}
				Rule::font_size_number => {
					let str = p.as_str();
					if str.is_empty() {
						o.push_str("[/font_size]");
					} else {
						o.push_str("[font_size=");
						o.push_str(str);
						o.push_str("]");
					}
				}
				Rule::variable_substitution_identifier => {
					// TODO
					o.push_str(p.as_str());
				}
			}
		}

		Ok(o)
	}
}

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "grammars/alm.pest"]
pub struct AlmParser;

fn parse_link_or_img(pairs: Pairs<'_, Rule>) -> (String, String) {
	let mut text = String::new();
	let mut url = String::new();
	for p in pairs.into_iter() {
		match p.as_rule() {
			Rule::link | Rule::image => {
				return parse_link_or_img(p.into_inner());
			}
			Rule::link_text | Rule::image_alt => {
				text = p.as_str().to_owned();
			}
			Rule::link_url | Rule::image_url => {
				url = p.as_str().to_owned();
			}
			_ => {}
		}
	}
	return (text, url);
}
