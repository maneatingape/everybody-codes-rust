use everybody_codes::event2025::quest07::*;

const EXAMPLE1: &str = "\
Oronris,Urakris,Oroneth,Uraketh

r > a,i,o
i > p,w
n > e,r
o > n,m
k > f,r
a > k
U > r
e > t
O > r
t > h";

const EXAMPLE2: &str = "\
Xanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax

r > v,e,a,g,y
a > e,v,x,r
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i";

const EXAMPLE3: &str = "\
Khara,Xaryt,Noxer,Kharax

r > v,e,a,g,y
a > e,v,x,r,g
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE1), "Oroneth");
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE2), 23);
}

#[test]
fn part3_test() {
    assert_eq!(part3(EXAMPLE3), 1154);
}
