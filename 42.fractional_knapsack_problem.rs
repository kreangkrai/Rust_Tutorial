//Fractional Knapsack Problem With Rust Programing Language
#[derive(Debug)]
struct Product {
    product: &'static str,
    value: f32,
    weight: f32,
}
#[allow(dead_code)]
#[derive(Debug)]
struct BOX {
    boxs: &'static str,
    weight: f32,
    products: Option<Vec<Product>>,
}
#[derive(Debug)]
struct BOXS {
    boxs: Vec<BOX>,
}
#[allow(unused_variables)]
impl<'a, 'b> BOXS {
    fn new(boxs: BOXS) -> Self {
        boxs
    }
    pub fn fractional_knapsack(&'a mut self,products: &'b mut Vec<Product>) -> (&'a Vec<BOX>, &'b Vec<Product>) {
        for b in self.boxs.iter_mut() {
            let mut sum_v = 0f32;
            let mut sum_w = 0f32;
            let mut box_products: Vec<Product> = vec![];
            for p in products.into_iter() {
                if p.value > 0f32 && p.weight > 0f32 && sum_w < b.weight {
                    let mut ratio = 1f32;
                    if (p.weight + sum_w) > b.weight {
                        ratio = (b.weight - sum_w) / p.weight;
                    }
                    let value = f32::trunc(ratio * p.value);
                    let weight = f32::trunc(ratio * p.weight);
                    sum_v += value;
                    sum_w += weight;
                    p.value -= value;
                    p.weight -= weight;

                    box_products.push(Product {
                        product: p.product,
                        value: value,
                        weight: weight,
                    });
                }
            }
            b.products = Some(box_products);
        }
        (&self.boxs, products)
    }
}
fn main() {
    let mut products: Vec<Product> = vec![];
    products.push(Product {
        product: "Product_1",
        value: 385f32,
        weight: 55f32,
    });
    products.push(Product {
        product: "Product_2",
        value: 405f32,
        weight: 45f32,
    });
    products.push(Product {
        product: "Product_3",
        value: 600f32,
        weight: 60f32,
    });
    products.push(Product {
        product: "Product_4",
        value: 320f32,
        weight: 40f32,
    });

    let mut boxs: Vec<BOX> = vec![];
    boxs.push(BOX {
        boxs: "BOX1",
        weight: 25f32,
        products: None,
    });
    boxs.push(BOX {
        boxs: "BOX2",
        weight: 20f32,
        products: None,
    });
    boxs.push(BOX {
        boxs: "BOX3",
        weight: 15f32,
        products: None,
    });
    boxs.push(BOX {
        boxs: "BOX4",
        weight: 15f32,
        products: None,
    });
    boxs.push(BOX {
        boxs: "BOX5",
        weight: 22f32,
        products: None,
    });
    boxs.push(BOX {
        boxs: "BOX6",
        weight: 18f32,
        products: None,
    });
    boxs.push(BOX {
        boxs: "BOX7",
        weight: 25f32,
        products: None,
    });
    boxs.push(BOX {
        boxs: "BOX8",
        weight: 30f32,
        products: None,
    });
    boxs.push(BOX {
        boxs: "BOX9",
        weight: 30f32,
        products: None,
    });
    
    // sort ratio of value per weight by desc
    products.sort_by(|a, b| (b.value / b.weight).total_cmp(&(a.value / a.weight)));

    let mut box_product = BOXS::new(BOXS { boxs: boxs });
    let (boxs, products) = box_product.fractional_knapsack(&mut products);

    for b in boxs.iter(){
        println!("======== {} Max Weight {} =======",b.boxs,b.weight);
        println!("Product   Weight    Value");
        for p in b.products.iter(){

            for a in p.iter(){
            println!("{}   {}      {}",a.product,a.weight,a.value);
            }
        }
        
    }
    println!();
    println!("======== Remain ========");
    println!("Product    Weight    Value");
    for p in products.into_iter(){
        println!("{}   {}        {}", p.product,p.weight,p.value);
    } 
}
