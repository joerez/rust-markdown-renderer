#![feature(proc_macro_hygiene)]
use clap::{clap_app, crate_version};
use maud::html;
use pulldown_cmark::{html::push_html, Event, Parser};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let clap = clap_app!( mrdend => 
                            (version:crate_version!())
                            (author:"Joe Rezendes")
                            (about:"Renders markdown you like")
                            (@arg input: +required "Sets the input markdown file")
                            (@arg wrap: -w "Wrap in html")
                            (@arg event: -e "Print event")
                            (@arg css: --css +takes_value "Link to css")
                            (@arg file: --file +takes_value "File name to create")
    )                       
    .get_matches();

    let infile =
        std::fs::read_to_string(clap.value_of("input").unwrap()).expect("could not read file");

    let mut res = String::new();
    let ps = Parser::new(&infile);

    let ps : Vec<Event> = ps.into_iter().collect();
    if clap.is_present("event") {
        for p in &ps {
            println!("{:?}", p);
        }
    }

    push_html(&mut res, ps.into_iter());


    if clap.is_present("wrap"){
        res = wrap_html(&res,clap.value_of("css"));

        create_file("test_two".to_string(), res);
    }

    // println!("{}", res);
}

fn wrap_html(s:&str,css:Option<&str>)->String{
    let res = html!{
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                @if let Some(s) = css {
                    link rel="stylesheet" type="text/css" href=(s) {}
                }
            }
            body {
                (maud::PreEscaped(s))
            }
        }
    };
    res.into_string()
}

fn create_file(title: String, html: String) -> std::io::Result<()> {
    let mut file = File::create(title + ".html")?;
    file.write_all(html.as_bytes())?;
    Ok(())
}