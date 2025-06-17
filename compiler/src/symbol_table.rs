use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolKind {
    Variable,
    Function,
    BuiltInFunction,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    // Potentially: pub scope: ScopeId (for future scoped tables)
    // Potentially: pub data_type: Option<String> (if type info is stored)
}

#[derive(Debug)]
pub struct SymbolTable {
    // For MVP, a single global scope.
    // Later, this could be a stack of HashMaps or a more complex structure.
    pub symbols: HashMap<String, Symbol>,
    // Potentially: parent: Option<Box<SymbolTable>> for nested scopes
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, kind: SymbolKind) -> Result<(), String> {
        if self.symbols.contains_key(&name) {
            Err(format!("Symbol '{}' is already defined in the current scope.", name))
        } else {
            let symbol = Symbol {
                name: name.clone(),
                kind,
            };
            self.symbols.insert(name, symbol);
            Ok(())
        }
    }

    pub fn resolve(&self, name: &String) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_and_resolve_symbol() {
        let mut st = SymbolTable::new();
        let name = "x".to_string();
        let kind_var = SymbolKind::Variable;
        let kind_fn = SymbolKind::Function;
        let kind_builtin = SymbolKind::BuiltInFunction;

        assert!(st.define(name.clone(), kind_var.clone()).is_ok(), "Failed to define new variable symbol.");
        match st.resolve(&name) {
            Some(symbol) => {
                assert_eq!(symbol.name, name);
                assert_eq!(symbol.kind, kind_var);
            }
            None => panic!("Failed to resolve defined variable symbol."),
        }
        st.symbols.clear(); // Clear for next test case within this func

        assert!(st.define("my_func".to_string(), kind_fn.clone()).is_ok(), "Failed to define new function symbol.");
        match st.resolve(&"my_func".to_string()) {
            Some(symbol) => {
                assert_eq!(symbol.name, "my_func");
                assert_eq!(symbol.kind, kind_fn);
            }
            None => panic!("Failed to resolve defined function symbol."),
        }
        st.symbols.clear();

        assert!(st.define("print".to_string(), kind_builtin.clone()).is_ok(), "Failed to define new built-in function symbol.");
        match st.resolve(&"print".to_string()) {
            Some(symbol) => {
                assert_eq!(symbol.name, "print");
                assert_eq!(symbol.kind, kind_builtin);
            }
            None => panic!("Failed to resolve defined built-in function symbol."),
        }
    }

    #[test]
    fn test_define_duplicate_symbol() {
        let mut st = SymbolTable::new();
        let name = "y".to_string();
        st.define(name.clone(), SymbolKind::Variable).unwrap();

        let result_fn = st.define(name.clone(), SymbolKind::Function);
        assert!(result_fn.is_err(), "Should not allow redefinition of symbol (var as fn).");
        assert_eq!(result_fn.unwrap_err(), "Symbol 'y' is already defined in the current scope.");

        let result_builtin = st.define(name.clone(), SymbolKind::BuiltInFunction);
        assert!(result_builtin.is_err(), "Should not allow redefinition of symbol (var as built-in).");
        assert_eq!(result_builtin.unwrap_err(), "Symbol 'y' is already defined in the current scope.");
    }

    #[test]
    fn test_resolve_undefined_symbol() {
        let st = SymbolTable::new();
        let name = "z".to_string();
        assert!(st.resolve(&name).is_none(), "Should not resolve undefined symbol.");
    }
}
