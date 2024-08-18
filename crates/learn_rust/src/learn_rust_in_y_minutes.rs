// 从 https://learnxinyminutes.com/docs/rust/ 翻译而来

#![allow(clippy::all)]

// 这是一个单行注释
// 以及连续的单行注释

/* 多行注释 Block comments
/* 可以嵌套 */   */

/// 文档注释，支持 markdown
/// # Examples
///
/// ```rs
/// let x = 5;
/// ```
///

////////////////
// 1. 基本语法 //
///////////////

#[allow(dead_code)]
// 函数定义
// i32 是 32 位有符号整数
fn add2(x: i32, y: i32) -> i32 {
  // 隐式返回（没有分号）
  x + y
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
pub fn main() {
  // 数字 //

  // 不可变变量（immutable）
  let x: i32 = 1;

  // 整型/浮点型后缀
  let y: i32 = 13i32;
  let f: f64 = 1.3f64;

  // 类型推导
  // 大多数情况下，Rust 编译器可以推导出变量的类型，所以
  // 你不需要显式地指定类型
  // 在本教程中，显式指定类型是为了演示，类型推导可以在大多数情况下解决问题
  let implicit_x = 1;
  let implicit_f = 1.3;

  // 运算
  let sum = x + y + 13;

  // 可变变量
  let mut mutable = 1;
  mutable = 4;
  mutable += 2;

  // 字符串 //

  // 字符串字面量
  let x: &str = "hello world!";

  // 打印
  println!("{} {}", f, x); // 1.3 hello world!

  // 一个 `String` - 在堆上分配的字符串
  // 储存为 `Vec<u8>` 并始终保存一个有效的且不以 `null` 结尾的 UTF-8 编码序列
  let s: String = "hello world".to_string();

  // 一个字符串切片 - 字符串的不可变视图
  // 是一个指向字符串的指针(`pointer`)和一个长度(`length`)
  // 它不包含字符串的内容，只是指向字符串缓冲区(`buffer`)的开头和长度的指针。
  // 静态分配或包含在另一个对象中，在这个例子中是 `s`
  // 字符串切片就像`Vec<T>`中的`&[u8]`视图。
  let s_slice: &str = &s;

  println!("{} {}", s, s_slice); // hello world hello world

  // 向量(`Vector`)/数组(`array`) //

  // 一个固定尺寸(`fixed-size`)的数组
  let four_ints: [i32; 4] = [1, 2, 3, 4];

  // 一个动态数字(向量)
  let mut vector: Vec<i32> = vec![1, 2, 3, 4];
  vector.push(5);

  // 一个切片 - 向量或数组的不可变视图(`immutable view`)
  let slice: &[i32] = &vector;

  // 使用 {:?} 打印调试信息，译者按：这里使用 {:?} 而不是 {} 是因为 {:?} 可以打印任意实现了 debug trait 的类型，而 {} 只能打印实现了 std::fmt::Display trait 的类型
  println!("{:?} {:?}", four_ints, slice); // [1, 2, 3, 4] [1, 2, 3, 4, 5]

  // 元组(`Tuple`) //

  // 元组是一组可能不同类型的固定大小的集合(set)
  let x: (i32, &str, f64) = (1, "hello", 3.4);

  // 解构赋值
  let (a, b, c) = x;
  println!("{} {} {}", a, b, c); // 1 hello 3.4

  // 索引
  println!("{}", x.1); // hello

  /////////////
  // 2. 类型 ///
  /////////////

  // 结构体(`struct`) //
  struct Pointer {
    x: i32,
    y: i32,
  }

  let origin: Pointer = Pointer { x: 0, y: 0 };

  // 具有未命名字段的结构体，称为”元组结构体“（tuple struct）
  struct Pointer2(i32, i32);

  let origin2: Pointer2 = Pointer2(0, 0);

  // 基础的类 c 的枚举
  enum Direction {
    Left,
    Right,
    Up,
    Down,
  }

  let up: Direction = Direction::Up;

  // 带字段的枚举
  // 如果你想让某些字段可选，可以使用标准库中的 `Option` 枚举
  enum OptionalI32 {
    AnI32(i32),
    Nothing,
  }

  let two: OptionalI32 = OptionalI32::AnI32(2);
  let nothing = OptionalI32::Nothing;

  // 泛型(`Generics`) //

  struct Foo<T> {
    bar: T,
  }

  // 这个枚举在标准库中叫做 `Option<T>`
  // 一般 Option 被用在可能包含空值的场景
  enum Optional<T> {
    SomeVal(T),
    NoVal,
  }

  // 方法(`Methods`) //

  impl<T> Foo<T> {
    // 一个方法显式得到 self 参数
    fn bar(&self) -> &T {
      &self.bar
    }
    fn bar_mut(&mut self) -> &mut T {
      //       ^^^^ self 被可变借用
      &mut self.bar
    }
    fn into_bar(self) -> T {
      //        ^^^^ self 被消耗
      self.bar
    }
  }

  let a_foo = Foo { bar: 1 };
  println!("{}", a_foo.bar()); // 1

  // 特性(`Traits`) 在其他语言被称为 `interface` //

  trait Frobnicate<T> {
    fn frobnicate(self) -> Option<T>;
  }

  impl<T> Frobnicate<T> for Foo<T> {
    fn frobnicate(self) -> Option<T> {
      Some(self.bar)
    }
  }

  let another_foo = Foo { bar: 1 };
  println!("{:?}", another_foo.frobnicate()); // Some(1)

  // 函数的类型签名 //

  fn fibonacci(n: u32) -> u32 {
    match n {
      0 => 1,
      1 => 1,
      _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
  }

  type FunctionPointer = fn(u32) -> u32;

  let fib: FunctionPointer = fibonacci;
  println!("Fib {}", fib(4)); // 5

  ////////////////
  // 3. 模式匹配 //
  ///////////////

  let foo = OptionalI32::AnI32(1);
  match foo {
    OptionalI32::AnI32(n) => println!("这是一个 i32: {}", n),
    OptionalI32::Nothing => println!("这是一个 nothing"),
  }

  // 高级模式匹配
  struct FooBar {
    x: i32,
    y: OptionalI32,
  }
  let bar = FooBar { x: 15, y: OptionalI32::AnI32(32) };

  match bar {
    FooBar { x: 0, y: OptionalI32::AnI32(0) } => println!("数字都是 0。"),
    FooBar { x: n, y: OptionalI32::AnI32(m) } if n == m => {
      println!("这两个数字相等。");
    }
    FooBar { x: n, y: OptionalI32::AnI32(m) } => {
      println!("这两个数字不相等：{}, {}", n, m);
    }
    FooBar { x: _, y: OptionalI32::Nothing } => println!("第二个数字是 nothing"),
  }

  //////////////////////////////
  // 4. 控制流(`control flow`) //
  /////////////////////////////

  // for 循环(`loop`)/迭代(`iteration`)
  let array = [1, 2, 3];
  for i in array {
    println!("{}", i);
  }

  // 范围(`Range`)
  for i in 0u32..10 {
    println!("{}", i);
  }
  // 打印 0 到 9

  // `if`
  if 1 == 1 {
    println!("数学存在！");
  } else {
    println!("数学不存在了...");
  }

  // `if` 作为一个表达式
  let value = if true { "good" } else { "bad" };

  // `while` 循环
  while 1 == 1 {
    println!("宇宙运转正常...");
    // break 退出循环，可以减少无意义的迭代
    break;
  }

  // 无限循环
  loop {
    println!("再来一次！");
    // break 退出循环
    break;
  }

  //////////////////////
  // 5. 内存安全 & 指针 //
  /////////////////////

  // 拥有(`Owned`)的指针 – 一次只有一个变量可以“拥有”这个指针
  let mut mine: Box<i32> = Box::new(32);
  *mine = 5;
  // 现在 `now_its_mine` 拥有这个指针。另一个说法是，`mine` 的所有权被转移(`move`)了
  let mut now_its_mine = mine;
  *now_its_mine += 2;

  println!("{}", now_its_mine); // 7
                                // println!("{}", mine); // 编译错误 因为 `mine` 的所有权已经被转移

  // 引用(`References`) – 引用其他数据的不可变指针
  // 当引用某个值时，我们说这个值被借用(`Borrowed`)了
  let mut var = 4;
  var = 3;
  let ref_var: &i32 = &var;

  println!("{}", var); // 3
  println!("{}", *ref_var); // 3
                            // *ref_var = 6 // 这个不会通过编译，因为 ref_var 是不可变的

  // 可变引用 (`mutable references`) – 引用其他数据的可变指针
  let mut var2 = 4;
  let ref_var2: &mut i32 = &mut var2;
  // * 解引用
  *ref_var2 += 2;
  println!("{}", *ref_var2); // 6
}
