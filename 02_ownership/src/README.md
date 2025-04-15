## 📌 라이프타임이 필요한 이유

러스트는 아래와 같은 **Dangling Reference**(죽은 값의 참조)를 컴파일 타임에 막습니다:

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x; // ❌ x는 이 블록을 벗어나면 사라짐!
    }

    // println!("r: {}", r); // 컴파일 에러! r은 x를 참조하는데 x는 죽었음
}
```

C/C++ 같은 언어에서는 런타임에 죽은 메모리를 참조하다가 버그/크래시가 나지만,  
**러스트는 애초에 컴파일 에러로 막아줍니다.**

---

## 📘 기본 예제: 명시적 라이프타임

```rust
// x와 y는 같은 라이프타임 'a 안에 살아 있어야 함
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```rust
fn main() {
    let s1 = String::from("abcd");
    let s2 = String::from("xyz");

    let result = longest(s1.as_str(), s2.as_str());
    println!("Longest: {}", result);
}
```

---

## 👀 컴파일러는 대부분의 경우 생략 가능 (Lifetime Elision)

아래와 같이 **간단한 함수**에서는 명시 안 해도 됩니다:

```rust
fn print(s: &str) {
    println!("{}", s);
}
```

하지만 **두 개 이상의 참조가 들어오고, 그 중 하나를 반환할 때**는 명시해야 합니다.  
왜냐하면 **“반환값이 어떤 입력과 연결되어 있는지”**를 러스트가 추론할 수 없기 때문이에요.

---

## 🧠 비유로 이해하는 라이프타임

> "라이프타임은 대출 만기일이다."

- `&T`: 책을 빌리는 것 (읽기만 가능)
- `&mut T`: 책에 메모도 가능한 대출 (단 한 명만 가능)
- `'a`: 책을 빌릴 수 있는 유효한 기간

```rust
fn borrow_book<'a>(book: &'a str) -> &'a str {
    book
}
```

> `book`을 빌린 사람은 `'a` 기간 안에 돌려줘야 한다는 약속

---

## 🧪 연습 예제

```rust
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}
```

이런 간단한 참조 함수에는 라이프타임을 생략해도 컴파일러가 자동으로 추론해줍니다.

---

## 🔑 핵심 요약

| 개념 | 의미 |
|------|------|
| 라이프타임 `'a` | 참조가 유효한 범위(스코프) |
| `&T` | 불변 참조 |
| `&mut T` | 가변 참조 (단일 소유만 허용) |
| 명시적 라이프타임 | 컴파일러가 추론하지 못할 때 직접 지정 |

---
