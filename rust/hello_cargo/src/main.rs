fn main() {
    println!("Hello, world!");

    let ckan = ckanaction::CKAN::builder().url("http://localhost:5000").build();

    let result = ckan.package_list()
       .limit(5)
       .call();
    println!("{}", result);

   
}
