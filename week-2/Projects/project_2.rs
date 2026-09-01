fn main(){
	let toshiboqty = 2;
	let macqty = 1;
	let hpqty = 3;
	let dellqty = 3;
	let acerqty = 1;

	let toshiboamt = 450_000;
	let macamt = 1_500_000;
	let hpamt = 750_000;
	let dellamt = 2_850_000;
    let aceramt = 250_000;

    let sum = toshiboamt + macamt + hpamt + dellamt + aceramt;

    let qty = toshiboqty + macqty + hpqty + dellqty + acerqty;

    let average = sum/qty;
     println!("The average amount of the current sales record is {}", average );
}