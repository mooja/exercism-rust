static BOOK_PRICE: u32 = 800;

static DISCOUNTS_FOR_SIZE: [f32; 6] = [
    0.0, 0.0, 0.05, 0.1, 0.2, 0.25
];

#[derive(Default, Clone)]
struct BookGroup {
    books: Vec<u32>
}

impl BookGroup {
    pub fn new() -> Self {
        BookGroup {
            books: vec![0; 5]
        }
    }

    pub fn num_books(&self) -> u32 {
        self.books.iter().map(|&bk_count| bk_count).sum::<u32>()
    }

    pub fn price(&self) -> u32 {
        let max_price = self.num_books() * BOOK_PRICE;
        let discount = (max_price as f32 * DISCOUNTS_FOR_SIZE[self.num_books() as usize]) as u32;
        max_price - discount
    }
}

fn books_in_basket(basket: &Vec<u32>) -> u32 {
    basket.iter().sum()
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let basket = {
        let mut basket = vec![0; 5];
        for &b in books {
            basket[b as usize - 1] += 1;
        }

        basket
    };

    let max_basket_price = basket
        .iter()
        .map(|&bk_count| bk_count * BOOK_PRICE)
        .sum::<u32>();
    let mut lowest_price_candidate = max_basket_price;

    for group_size in 1..=5 {
        let mut basket = basket.clone();
        let mut total = 0;

        while books_in_basket(&basket) != 0 {
            let mut current_group = BookGroup::new();

            for i in 0..basket.len() {
                if basket[i] > 0 {
                    basket[i] -= 1;
                    current_group.books[i] += 1;
                }

                if current_group.num_books() == group_size {
                    break;
                }
            } 

            total += current_group.price();
        }

        if total < lowest_price_candidate {
            lowest_price_candidate = total;
        }
    }
    
    lowest_price_candidate
}
