use proc_macro::{self,  TokenStream};
use syn::{
    parse_macro_input,
    spanned::Spanned,
     DeriveInput,

};

use quote::{quote, ToTokens};

#[proc_macro_derive(PgCurdStruct, attributes(TableName,CacheName))]
pub fn derive_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;
    let attrs = &ast.attrs;
    let mut table_name: Option<String> = None;
    let mut cache_name: Option<String> = None;
    for attr in attrs {
        match attr.parse_meta().unwrap() {
            syn::Meta::NameValue(val) => {
                if val.path.is_ident("TableName") {
                    if let syn::Lit::Str(lit) = &val.lit {
                        table_name = Some(lit.value());
                    }
                }
                if val.path.is_ident("CacheName") {
                    if let syn::Lit::Str(lit) = &val.lit {
                        cache_name = Some(lit.value());
                    }
                }
                
            }
            _ => (),
        }
    }

    let table_name = table_name.expect("TableName attr not found");
    let cache_name=cache_name.expect("CacheName attr not found");
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let mut keys = Vec::new();
    let mut idents = Vec::new();
    let mut types = Vec::new();

    let mut select_sql=String::from("");
    for field in fields.named.iter() {
        let field_name: &syn::Ident = field.ident.as_ref().unwrap();
        let name: String = field_name.to_string();
        let literal_key_str = syn::LitStr::new(&name, field.span());
        let type_name = &field.ty;
        keys.push(quote! { #literal_key_str });
        idents.push(&field.ident);
        types.push(type_name.to_token_stream());
  
        let column_name = name.to_lowercase();

        select_sql=format!("{}{},",select_sql,column_name);
      
    }
    select_sql=select_sql[..select_sql.len()-1].to_string();

    // let select_sql=quote! {#(select_sql) };
    //let select_fields = quote! { concat!(#(#select_fields, ", "),*) };
    // let insert_fields = quote! { concat!(#(#insert_fields, ", "),*) };
    //let insert_fields = quote! { concat!(#(#keys = self.#idents, ", "),*) };
    //let update_fields = quote! { concat!(#(#update_fields, ", "),*) };
    let expanded = quote! {
        impl PgCurdStruct for #struct_name {
            fn get_table_name(&self)->&'static str{
                #table_name
            }
            fn get_cache_name(&self)->&'static str{
                #cache_name
            }
            fn select(&self) -> String {
                format!("SELECT {} FROM {} order by id ASC;",#select_sql, #table_name)
                
            }

            fn insert(&self) -> String {    
                let mut keys: String = String::new();
                let mut values: String = String::new();
                for (key, value) in serde_json::json!(self).as_object().unwrap() {
                    if(key.eq("id")) {
                        values.push_str("");
                    } else {
                        keys.push_str(format!("{}, ", key).as_str());
                        values.push_str(format!("'{}', ", value.to_string()).as_str());
                    }
                }
                // values = values.replace("\\", "");
                values = values.replace("\"", "");
                let last_values = &values[..values.len() - 2];
                let last_keys = &keys[..keys.len() - 2];          
                format!("INSERT INTO {}({}) VALUES({})", #table_name,last_keys, last_values)
            }

            fn update(&self,id: i32) -> String {
                let mut update:String=String::new();
                for (key, value) in serde_json::json!(self).as_object().unwrap() {
                    
                    if(key.eq("id")) {

                    } else {
                        update.push_str(format!("{} = '{}', ",key, value.to_string()).as_str());
                    }
                }
                // update = update.replace("\\", "");
                update = update.replace("\"", "");
               
                let last_update = &update[..update.len() - 2];
                format!("UPDATE {} SET {} WHERE id = {}", #table_name, last_update, id)
            }

            fn delete(&self,id: i32) -> String {
                format!("DELETE FROM {} WHERE id = {}", #table_name, id)
            }

            fn get_one_by_id(&self,id: i32) -> String {
                format!("SELECT * from {} where id ={}", #table_name, id)
            }

            // fn get_one_by_id<'a,'b>(&self,state: &'a DbState,id:i32) -> Result<Model> {
            //     #[allow(unused_assignments)]
            //     let mut sql=String::new();
            //     sql=format!("SELECT * from {} where id ={}",self.get_table_name(),id);
            //     let pool = get_conn(&state);
            //     let rows = sqlx::query_as::<_, Model>(&sql)
            //         .fetch_one(pool)
            //         .await
            //         .unwrap();
            //     Ok(rows)
            // }
            fn print(&self) {
                #(
                    eprintln!(
                        "key={key}, value={value}, type={type_name},INSERT INTO {table_name}",
                        key = #keys,
                        value = self.#idents,
                        type_name = stringify!(#types),
                        table_name = #table_name
                    );
                )*
            }
        }
    };
    expanded.into()
}