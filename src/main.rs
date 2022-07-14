use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "base.stpl")]
struct Base {
    title: &'static str,
    scripts: &'static [&'static str],
    stylesheets: &'static [&'static str],
    main: &'static str,
}

fn main() {
    generate_pages([
        ("index.html", index_template()),
        ("noctane.html", noctane_template()),
    ]);
}

fn generate_pages(pages: impl IntoIterator<Item = (&'static str, Base)>) {
    for (filename, template) in pages {
        generate_page(filename, template);
    }
}

fn generate_page(filename: &str, template: Base) {
    let mut minify_cfg = minify_html::Cfg::spec_compliant();
    minify_cfg.minify_css = true;
    minify_cfg.minify_js = false;

    std::fs::write(
        std::path::Path::new(filename),
        minify_html::minify(
            template.render_once().unwrap().as_bytes(),
            &minify_cfg,
        ),
    )
    .unwrap();
}

fn index_template() -> Base {
    Base {
        title: "norepi's Personal Site",
        scripts: &[],
        stylesheets: &["/base.css"],
        main: include_str!("html/fragments/index-main.html"),
    }
}

fn noctane_template() -> Base {
    Base {
        title: "Noctane for Web",
        scripts: &["/coi-serviceworker.min.js"],
        stylesheets: &["/base.css"],
        main: include_str!("html/fragments/noctane-main.html"),
    }
}
