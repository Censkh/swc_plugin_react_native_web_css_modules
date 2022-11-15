use std::path::Path;

use swc_common::DUMMY_SP;
use swc_core::{ecma::{ast::Ident, ast::Program, visit::FoldWith}, plugin::{
    plugin_transform,
    proxies::TransformPluginProgramMetadata,
}, quote};
use swc_core::ecma::ast::{ImportDecl, ImportSpecifier, ModuleItem};
use swc_core::ecma::visit::as_folder;
use swc_core::ecma::visit::VisitMut;
use swc_core::ecma::visit::VisitMutWith;

#[derive(Debug)]
struct CssModuleImport {
    import_path: String,
    ident: Ident,
}

#[derive(Default)]
pub struct TransformVisitor {
    css_modules: Vec<CssModuleImport>,
}

impl VisitMut for TransformVisitor {
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        let import = n.src.value.to_string();
        let import_path = Path::new(&import);
        if let Some(ext) = import_path.extension() {
            if ext == "css" || ext == "scss" {
                for specifier in &mut n.specifiers {
                    let name;
                    let ident;

                    match specifier {
                        ImportSpecifier::Named(specifier) => {
                            name = specifier.local.sym.to_string();
                            ident = specifier.local.clone();
                            specifier.local = Ident::new(format!("$_css_module_{}", name).into(), DUMMY_SP);
                        }
                        ImportSpecifier::Default(specifier) => {
                            name = specifier.local.sym.to_string();
                            ident = specifier.local.clone();
                            specifier.local = Ident::new(format!("$_css_module_{}", name).into(), DUMMY_SP);
                        }
                        ImportSpecifier::Namespace(specifier) => {
                            name = specifier.local.sym.to_string();
                            ident = specifier.local.clone();
                            specifier.local = Ident::new(format!("$_css_module_{}", name).into(), DUMMY_SP);
                        }
                    };

                    self.css_modules.push(CssModuleImport {
                        ident: ident,
                        import_path: import.clone(),
                    });
                }
            }
        }
    }

    fn visit_mut_module_items(&mut self, n: &mut Vec<ModuleItem>) {
        n.visit_mut_children_with(self);

        if self.css_modules.len() > 0 {
            n.push(ModuleItem::Stmt(quote!(r###"var $_css_modules_rnw_process = (cssModule) => {
            return Object.keys(cssModule).reduce((processed, key) => {
                const value = cssModule[key];
                processed[key] = {$$css: true, [value]: value};
                return processed;
            }, {});
        }"### as Stmt)));
        }

        for css_module in &self.css_modules {
            let value_ident = Ident::new(format!("$_css_module_{}", css_module.ident.sym.to_string()).into(), DUMMY_SP);
            n.push(ModuleItem::Stmt(quote!(r###"var $name = $_css_modules_rnw_process($value);"### as Stmt, name = css_module.ident.clone(), value = value_ident)));
        }

        self.css_modules.clear();
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor::default()))
}
