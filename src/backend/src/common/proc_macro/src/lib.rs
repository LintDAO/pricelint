use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, Token};

#[proc_macro]
pub fn generate_service_trait(input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let name = parse_macro_input!(input as Ident);

    let lower_case = input_clone.to_string().to_lowercase();
    let create_fun_token = format!("{}_{}", "create_deafult", lower_case).parse().unwrap();
    let create_fun_ident = parse_macro_input!(create_fun_token as Ident);

    let update_fun_token = format!("{}_{}", "update", lower_case).parse().unwrap();
    let update_fun_ident = parse_macro_input!(update_fun_token as Ident);

    let delete_fun_token = format!("{}_{}", "delete", lower_case).parse().unwrap();
    let delete_fun_ident = parse_macro_input!(delete_fun_token as Ident);

    let service = format!("{}{}", input_clone, "Service");
    let tt = service.parse().unwrap();
    let ident = parse_macro_input!(tt as Ident);
    quote! {
        pub trait #ident {
            const  MAP:&'static LocalKey<RefCell<ic_stable_structures::btreemap::BTreeMap<String, Context<#name>, Memory>>>;

            type Output;

            fn is_exist(principal: Principal) -> bool;

            fn #create_fun_ident() -> Option<Self::Output>;

            fn #delete_fun_ident(&self);

            fn #update_fun_ident(data:Context<Self::Output>);

            fn find_all()->Vec<Self::Output>;

            fn find_one_by_id(id:String)->Self::Output;

            fn find_list_by_id(id:String)->Vec<Self::Output>;

            fn find_one_by_principal(principal:Principal)->Self::Output;

            fn find_list_by_principal(principal:Principal)->Vec<Self::Output>;
        }

    }
    .into()
}

#[proc_macro]
pub fn generate_service_impl(input: TokenStream) -> TokenStream {
    let ServiceImplInput {
        type_name,
        map_name,
    } = parse_macro_input!(input as ServiceImplInput);

    let input_clone = type_name.clone();

    let lower_case = type_name.to_string().to_lowercase();

    let create_fun_token = format!("{}_{}", "create_deafult", lower_case).parse().unwrap();
    let create_fun_ident = parse_macro_input!(create_fun_token as Ident);


    let update_fun_token = format!("{}_{}", "update", lower_case).parse().unwrap();
    let update_fun_ident = parse_macro_input!(update_fun_token as Ident);


    let delete_fun_token = format!("{}_{}", "delete", lower_case).parse().unwrap();
    let delete_fun_ident = parse_macro_input!(delete_fun_token as Ident);

    let service = format!("{}{}", input_clone, "Service");
    let tt = service.parse().unwrap();
    let ident = parse_macro_input!(tt as Ident);
    quote! {
    impl #ident for #type_name {

            const  MAP:&'static LocalKey<RefCell<ic_stable_structures::btreemap::BTreeMap<String, Context<#type_name>, Memory>>>=&#map_name;

            type Output=#type_name;

            fn #create_fun_ident()->Option<Self::Output> {
                let context = Context::new(Self::Output::default());
                let ret=map_insert!(
                    Self::MAP,
                    context.owner.unwrap().to_string(),
                    context
                );
                ret.unwrap().context
            }



            fn #delete_fun_ident(id:String)->bool {
                let is_removed = MAP.with(|map| {
                    let mut bm = map.borrow_mut();
                    bm.remove(&id)
                });
                match is_removed {
                    None => false,
                    Some(_) => true,
                }
            }
            fn #update_fun_ident(data:Context<Self::Output>){
                let update = MAP.with(|map| {
                let mut bm = map.borrow_mut();
                     bm.insert(data.id.clone().unwrap(),data);
                });
                update
            }

            fn create_with_context(context:Self::Output){
               let context = Context::new(context);
                    let ret=map_insert!(
                        Self::MAP,
                        context.owner.unwrap().to_string(),
                        context
                    );
                ret.unwrap().context
            }

            fn find_all()->Vec<Self::Output>{
                MAP.with(|map| map.borrow_mut().values().cloned().into())
            }

            fn find_by_id(id:String)->Option<Self::Output>{
                MAP.with(|map| {
                    map.borrow_mut()
                        .find(|&(k, v)| v.id.unwrap() == id)
                        .iter()
                        .collect()
                })
            }



            fn find_one_by_principal(principal:Principal)-><Option<Self::Output>>{
                MAP.with(|map| {
                    map.borrow_mut()
                        .find(|&(k, v)| v.owner.unwrap() == principal)
                        .iter()
                        .collect()
                })
            }

            fn find_list_by_principal(principal:Principal)->Option<Vec<Self::Output>>{
                MAP.with(|map| {
                    map.borrow_mut()
                        .find(|&(k, v)| v.owner.unwrap() == principal)
                        .iter()
                        .collect()
                })
            }

    }}.into()
}
struct ServiceImplInput {
    type_name: Ident,
    map_name: Ident,
}

impl Parse for ServiceImplInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 解析第一个标识符
        let type_name = input.parse()?;
        // 期望逗号
        input.parse::<Token![,]>()?;
        // 解析第二个标识符
        let map_name = input.parse()?;
        Ok(ServiceImplInput {
            type_name,
            map_name,
        })
    }
}
