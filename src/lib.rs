/// This file is generated. Do not edit it by hand.
/// This file is generated. Do not edit it by hand.
extern crate parity_webapp;

use std::default::Default;
use std::collections::HashMap;
use parity_webapp::{WebApp, File, Info};

pub struct App {
  files: HashMap<&'static str, File>,
}

impl WebApp for App {
  fn file(&self, path: &str) -> Option<&File> {
    self.files.get(path)
  }
  fn info(&self) -> Info {
    Info {
      name: "Maker OTC".to_owned(),
      version: "0.1.0".to_owned(),
      author: "Ethcore <admin@ethcore.io>".to_owned(),
      description: "Maker on-chain OTC market.".to_owned(),
      icon_url: "icon.png".to_owned(),
    }
  }
}

impl Default for App {
  fn default() -> Self {
    let files = {
      let mut files = HashMap::new();
      files.insert("17eaab911f93da07e71f005ca94b4b2919148579.js", File { path: "17eaab911f93da07e71f005ca94b4b2919148579.js", content_type: "application/javascript", content: include_bytes!("./web/17eaab911f93da07e71f005ca94b4b2919148579.js") });
      files.insert("78a7a91649e6d394803348f13bef32564bf4b75c.css", File { path: "78a7a91649e6d394803348f13bef32564bf4b75c.css", content_type: "text/css", content: include_bytes!("./web/78a7a91649e6d394803348f13bef32564bf4b75c.css") });
      files.insert("apple-touch-icon-114x114.png", File { path: "apple-touch-icon-114x114.png", content_type: "image/png", content: include_bytes!("./web/apple-touch-icon-114x114.png") });
      files.insert("apple-touch-icon-120x120.png", File { path: "apple-touch-icon-120x120.png", content_type: "image/png", content: include_bytes!("./web/apple-touch-icon-120x120.png") });
      files.insert("apple-touch-icon-144x144.png", File { path: "apple-touch-icon-144x144.png", content_type: "image/png", content: include_bytes!("./web/apple-touch-icon-144x144.png") });
      files.insert("apple-touch-icon-152x152.png", File { path: "apple-touch-icon-152x152.png", content_type: "image/png", content: include_bytes!("./web/apple-touch-icon-152x152.png") });
      files.insert("apple-touch-icon-57x57.png", File { path: "apple-touch-icon-57x57.png", content_type: "image/png", content: include_bytes!("./web/apple-touch-icon-57x57.png") });
      files.insert("apple-touch-icon-60x60.png", File { path: "apple-touch-icon-60x60.png", content_type: "image/png", content: include_bytes!("./web/apple-touch-icon-60x60.png") });
      files.insert("apple-touch-icon-72x72.png", File { path: "apple-touch-icon-72x72.png", content_type: "image/png", content: include_bytes!("./web/apple-touch-icon-72x72.png") });
      files.insert("apple-touch-icon-76x76.png", File { path: "apple-touch-icon-76x76.png", content_type: "image/png", content: include_bytes!("./web/apple-touch-icon-76x76.png") });
      files.insert("favicon-128.png", File { path: "favicon-128.png", content_type: "image/png", content: include_bytes!("./web/favicon-128.png") });
      files.insert("favicon-16x16.png", File { path: "favicon-16x16.png", content_type: "image/png", content: include_bytes!("./web/favicon-16x16.png") });
      files.insert("favicon-196x196.png", File { path: "favicon-196x196.png", content_type: "image/png", content: include_bytes!("./web/favicon-196x196.png") });
      files.insert("favicon-32x32.png", File { path: "favicon-32x32.png", content_type: "image/png", content: include_bytes!("./web/favicon-32x32.png") });
      files.insert("favicon-96x96.png", File { path: "favicon-96x96.png", content_type: "image/png", content: include_bytes!("./web/favicon-96x96.png") });
      files.insert("favicon.ico", File { path: "favicon.ico", content_type: "image/x-icon", content: include_bytes!("./web/favicon.ico") });
      files.insert("index.html", File { path: "index.html", content_type: "text/html", content: include_bytes!("./web/index.html") });
      files.insert("logo.png", File { path: "logo.png", content_type: "image/png", content: include_bytes!("./web/logo.png") });
      files.insert("mstile-144x144.png", File { path: "mstile-144x144.png", content_type: "image/png", content: include_bytes!("./web/mstile-144x144.png") });
      files.insert("mstile-150x150.png", File { path: "mstile-150x150.png", content_type: "image/png", content: include_bytes!("./web/mstile-150x150.png") });
      files.insert("mstile-310x150.png", File { path: "mstile-310x150.png", content_type: "image/png", content: include_bytes!("./web/mstile-310x150.png") });
      files.insert("mstile-310x310.png", File { path: "mstile-310x310.png", content_type: "image/png", content: include_bytes!("./web/mstile-310x310.png") });
      files.insert("mstile-70x70.png", File { path: "mstile-70x70.png", content_type: "image/png", content: include_bytes!("./web/mstile-70x70.png") });
      files.insert("packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.eot", File { path: "packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.eot", content_type: "application/vnd.ms-fontobject", content: include_bytes!("./web/packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.eot") });
      files.insert("packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.svg", File { path: "packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.svg", content_type: "image/svg+xml", content: include_bytes!("./web/packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.svg") });
      files.insert("packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.ttf", File { path: "packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.ttf", content_type: "application/x-font-ttf", content: include_bytes!("./web/packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.ttf") });
      files.insert("packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.woff", File { path: "packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.woff", content_type: "application/font-woff", content: include_bytes!("./web/packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.woff") });
      files.insert("packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.woff2", File { path: "packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.woff2", content_type: "application/font-woff2", content: include_bytes!("./web/packages/twbs_bootstrap/dist/fonts/glyphicons-halflings-regular.woff2") });
      files
    };
    App {
      files: files,
    }
  }
}
