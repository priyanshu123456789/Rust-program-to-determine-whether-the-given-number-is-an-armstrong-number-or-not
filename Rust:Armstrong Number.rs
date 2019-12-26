fn main() {

let num:u32=153;
let mut _orignal_no:u32=0;
let mut _rem:u32=0;
let mut res:u32=0;

_orignal_no=num;

while _orignal_no != 0{
   _rem=_orignal_no % 10;
   res= res+(_rem*_rem*_rem);
   _orignal_no=_orignal_no/10;
}
if res==num{
println!("The number {} is an Armstrong number",num);} 
else {
println!("The number {} is not an Armstrong number",num);} }
