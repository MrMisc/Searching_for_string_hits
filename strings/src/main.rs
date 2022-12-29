fn main() {
    let stringo1 = "sklfjsd slfj sdljflskdjfs ";
    let stringo2 = "rabbits cross the road in america";
    let stringo3 = "chicken does not cross the road in malaysia";
    let stringo4 = "goats do not cross the roads i;n argentina";
    let stringo5 = "cows cross everything in europe";
    let stringo6 = "chickens do not fly in india";
    let stringo7 = "cows do not eat in america";
    let stringo8 = "rabbits are not eaten in india";
    let stringo9 = "flying is not allowed in antarctica";
    let listofhits = vec!["cows", "chicken", "rabbits"]; //assume this to be user input
    let listofhits2 = vec!["malaysia", "india"]; //same here. more user input
    let allstrings: &[&str] = &vec![
        stringo1, stringo2, stringo3, stringo4, stringo5, stringo6, stringo7, stringo8, stringo9,
    ];

    let output = allstrings
        .into_iter()
        .filter(|x| {
            let x = x.to_lowercase();
            listofhits2.iter().any(|needle| x.contains(needle))
                && listofhits.iter().any(|needle| x.contains(needle))
        })
        .collect::<Vec<_>>();
    println!("{:?}", output);
}
