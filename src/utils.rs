pub fn calculate_price_change(new_price: f64, old_price: f64) -> (f64, f64) {
    let change_in_price = new_price - old_price;
    let percentage_change = if old_price != 0.0 {
        (change_in_price / old_price) * 100.0
    } else {
        0.0 
    };

    (change_in_price, percentage_change)
}