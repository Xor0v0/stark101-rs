// fn part1() {
//     // 1. print FieldElement
//     println!("{}", FieldElement::new(3221225472) + FieldElement::new(10));

//     // 2. construct a list of length 1023 whose first two elements are 1 and 3141592
//     let mut elements = vec![FieldElement::one(), FieldElement::new(3141592)];
//     while elements.len() < 1023 {
//         elements.push(elements[elements.len() - 1].pow(2) + elements[elements.len() - 2].pow(2));
//     }
//     assert!(elements.len() == 1023);
//     assert!(elements[0] == FieldElement::one());
//     println!("{}", elements[0].pow(2) + elements[1].pow(2));
//     for i in 2..elements.len() {
//         assert!(elements[i] == elements[i - 1].pow(2) + elements[i - 2].pow(2));
//     }
//     println!("{}", elements[1022]);
//     assert!(elements[1022] == FieldElement::new(2338775057));
//     println!("Part 1 passed");

//     // 3. interpolate the polynomial that passes through the points (0, 1), (1, 3141592) ...
//     // find a subgroup of size 1024 (给定有限域的所构成的乘法群是一个阶为3*2^30的循环群，因此一定存在2^i大小的乘法子群)
//     let g = FieldElement::generator().pow(3 * (1 << 20));
//     let mut G = vec![];
//     for i in 0..1024 {
//         G.push(g.pow(i));
//     }
//     assert!(g.is_order(1024));
//     let mut b = FieldElement::one();
//     for i in 0..1023 {
//         assert!(G[i] == b);
//         b = b * g;
//         assert!(b != FieldElement::one());
//     }
//     if b * g == FieldElement::one() {
//         println!("g is a generator of the subgroup of size 1024");
//     } else {
//         println!("g is not a generator of the subgroup of size 1024");
//     }

//     // 4. evaluating the polynomial on a larger domain
//     let w = FieldElement::generator().pow(3 * (1 << 30) / 8196);
//     let mut H = vec![];
//     for i in 0..8192 {
//         H.push(w.pow(i));
//     }
//     let mut eval_domain = vec![];
//     for i in 0..8192 {
//         eval_domain.push(w * H[i]);
//     }
// }

fn main() {
    println!("Hello, world!");
    // part1();
}
