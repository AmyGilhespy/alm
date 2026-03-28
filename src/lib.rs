use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

#[derive(Clone, Copy, Debug, Default)]
pub enum OutputFormat {
	#[default]
	Html,
	Godot,
}

pub struct Alm;

impl Alm {
	pub fn parse(string: &str, format: OutputFormat) -> Result<String, pest::error::Error<Rule>> {
		let pairs = AlmParser::parse(Rule::document, string)?;
		match format {
			OutputFormat::Html => f(pairs, "Html"),
			OutputFormat::Godot => f(pairs, "Godot"),
		}
	}
}

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "grammars/alm.pest"]
pub struct AlmParser;

fn f(pairs: Pairs<'_, Rule>, x: &str) -> Result<String, pest::error::Error<Rule>> {
	let mut o = String::new();

	for p in pairs.into_iter() {
		match p.as_rule() {
			Rule::WHITESPACE
			| Rule::NEWLINE
			| Rule::blank_line
			| Rule::code_line
			| Rule::link_text
			| Rule::link_url
			| Rule::image_alt
			| Rule::image_url
			| Rule::plain_text => {
				o.push_str(p.as_str());
			}
			Rule::document
			| Rule::block
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
			| Rule::paragraph
			| Rule::inline
			| Rule::code_span
			| Rule::superscript
			| Rule::subscript
			| Rule::color
			| Rule::font_size
			| Rule::variable_substitution => {
				o.push_str(f(p.into_inner(), x)?.as_str());
			}
			Rule::OMITTED_WHITESPACE
			| Rule::OMITTED_NEWLINE
			| Rule::frontmatter
			| Rule::frontmatter_line
			| Rule::code_info
			| Rule::blockquote_marker
			| Rule::bold_start
			| Rule::italic_start
			| Rule::underline_start
			| Rule::strikethrough_start
			| Rule::spoiler_start
			| Rule::code_span_start
			| Rule::superscript_start
			| Rule::subscript_start
			| Rule::link_start
			| Rule::image_start
			| Rule::color_start
			| Rule::font_size_start
			| Rule::EOI => {}
			Rule::h1_inner => {
				o.push_str(match x {
					"Godot" => "[h1]",
					"Html" => "<h1>",
					_ => panic!(),
				});
				o.push_str(p.as_str());
				o.push_str(match x {
					"Godot" => "[/h1]",
					"Html" => "</h1>",
					_ => panic!(),
				});
			}
			Rule::h2_inner => {
				o.push_str(match x {
					"Godot" => "[h2]",
					"Html" => "<h2>",
					_ => panic!(),
				});
				o.push_str(p.as_str());
				o.push_str(match x {
					"Godot" => "[/h2]",
					"Html" => "</h2>",
					_ => panic!(),
				});
			}
			Rule::h3_inner => {
				o.push_str(match x {
					"Godot" => "[h3]",
					"Html" => "<h3>",
					_ => panic!(),
				});
				o.push_str(p.as_str());
				o.push_str(match x {
					"Godot" => "[/h3]",
					"Html" => "</h3>",
					_ => panic!(),
				});
			}
			Rule::h4_inner => {
				o.push_str(match x {
					"Godot" => "[h4]",
					"Html" => "<h4>",
					_ => panic!(),
				});
				o.push_str(p.as_str());
				o.push_str(match x {
					"Godot" => "[/h4]",
					"Html" => "</h4>",
					_ => panic!(),
				});
			}
			Rule::h5_inner => {
				o.push_str(match x {
					"Godot" => "[h5]",
					"Html" => "<h5>",
					_ => panic!(),
				});
				o.push_str(p.as_str());
				o.push_str(match x {
					"Godot" => "[/h5]",
					"Html" => "</h5>",
					_ => panic!(),
				});
			}
			Rule::h6_inner => {
				o.push_str(match x {
					"Godot" => "[h6]",
					"Html" => "<h6>",
					_ => panic!(),
				});
				o.push_str(p.as_str());
				o.push_str(match x {
					"Godot" => "[/h6]",
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
			Rule::bold => {
				o.push_str(match x {
					"Godot" => "[b]",
					"Html" => "<b>",
					_ => panic!(),
				});
				o.push_str(f(p.into_inner(), x)?.as_str());
				o.push_str(match x {
					"Godot" => "[/b]",
					"Html" => "</b>",
					_ => panic!(),
				});
			}
			Rule::italic => {
				o.push_str(match x {
					"Godot" => "[i]",
					"Html" => "<i>",
					_ => panic!(),
				});
				o.push_str(f(p.into_inner(), x)?.as_str());
				o.push_str(match x {
					"Godot" => "[/i]",
					"Html" => "</i>",
					_ => panic!(),
				});
			}
			Rule::underline => {
				o.push_str(match x {
					"Godot" => "[u]",
					"Html" => "<u>",
					_ => panic!(),
				});
				o.push_str(f(p.into_inner(), x)?.as_str());
				o.push_str(match x {
					"Godot" => "[/u]",
					"Html" => "</u>",
					_ => panic!(),
				});
			}
			Rule::strikethrough => {
				o.push_str(match x {
					"Godot" => "[s]",
					"Html" => "<s>",
					_ => panic!(),
				});
				o.push_str(f(p.into_inner(), x)?.as_str());
				o.push_str(match x {
					"Godot" => "[/s]",
					"Html" => "</s>",
					_ => panic!(),
				});
			}
			Rule::spoiler => {
				// TODO
				o.push_str(match x {
					"Godot" => "[b]",
					"Html" => "<b>",
					_ => panic!(),
				});
				o.push_str(f(p.into_inner(), x)?.as_str());
				o.push_str(match x {
					"Godot" => "[/b]",
					"Html" => "</b>",
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
				// TODO
				o.push_str(match x {
					"Godot" => "[url=",
					"Html" => "<a href=\">",
					_ => panic!(),
				});
				o.push_str(p.as_str());
				o.push_str(match x {
					"Godot" => "]",
					"Html" => "\">",
					_ => panic!(),
				});
				o.push_str(p.as_str());
				o.push_str(match x {
					"Godot" => "[/url]",
					"Html" => "</a>",
					_ => panic!(),
				});
			}
			Rule::image => {
				// TODO
				o.push_str(match x {
					"Godot" => "[img]",
					"Html" => "<img alt=\"",
					_ => panic!(),
				});
				o.push_str(p.as_str());
				o.push_str(match x {
					"Godot" => "",
					"Html" => "\" src=\"",
					_ => panic!(),
				});
				o.push_str(p.as_str());
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
					o.push_str("[/color]");
				} else {
					o.push_str("[color=");
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
