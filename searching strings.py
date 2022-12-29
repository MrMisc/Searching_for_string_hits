

stringo1 = "sklfjsd slfj sdljflskdjfs "
stringo2 = "rabbits cross the road in america"
stringo3 = "chicken does not cross the road in malaysia"
stringo4 = "goats do not cross the roads in argentina"
stringo5 = "cows cross everything in europe"
stringo6 = "chickens do not fly in india"
stringo7 = "cows do not eat in america"
stringo8 = "rabbits are not eaten in india"
stringo9 = "flying is not allowed in antarctica"
listofhits = ["cows", "chicken", "rabbits"]
listofhits2 = ["malaysia", "india"]



allstrings = [stringo1,stringo2,stringo3,stringo4, stringo5, stringo6, stringo7, stringo8, stringo9]
#Program that returns boolean if 2 hits are found within list for a string
print([x for x in allstrings if  any(i in x.lower() for i in listofhits2) and any(i in x.lower() for i in listofhits)])