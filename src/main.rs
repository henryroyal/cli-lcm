fn main()
{
    let (a, b) = read_input();
    let result = lcm(a, b);
    println!("{}", result);
    std::process::exit(0);
}


fn read_input() -> (i64, i64) {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer);

    buffer = buffer.replace("\n", "");
    let mut split = buffer.split_whitespace();

    let mut input: Vec<i64> = Vec::new();
    for item in split {
        let int: i64 = item.parse().unwrap();
        input.push(int);
    }

    if input.len() != 2 {
        println!("input should be a pair of integers!");
        std::process::exit(1);
    }

    return (input[0], input[1]);
}


fn lcm(mut a: i64, mut b: i64) -> i64
{
    let largest_prime = largest_shared_prime(a, b);
    return (a * b) / largest_prime;
}


fn largest_shared_prime(mut a: i64, mut b: i64) -> i64
{
    let mut d = 0;
    loop {
        if a == b {
            return a;
        }

        if a == 1 || b == 1 {
            return 1;
        }

        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }
}