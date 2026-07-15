// 题目 1：单元测试基础与断言限制
//
// 目标：
// 1. 修复 `BoxDimension` 结构体的声明，使其能够被 `assert_eq!` 比较。
// 2. 补齐测试函数的属性宏，使其成为一个有效的测试用例，并在测试中通过断言。
//
// 运行验证命令：
// cargo test --bin testing_exercises exercise1

// TODO: 观察下面的编译错误。在断言宏中使用 `assert_eq!` 时对类型有什么要求？
// 请为 `BoxDimension` 补齐必要的属性，以允许进行相等比较和在失败时调试打印。
#[derive(PartialEq, Debug)]
pub struct BoxDimension {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl BoxDimension {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    pub fn volume(&self) -> u32 {
        self.width * self.height * self.depth
    }
}

// TODO: 单元测试模块通常使用什么条件编译属性，以避免测试代码进入生产包？
// 请在这里添加适当的条件编译属性，使该模块仅在测试配置下编译。
#[cfg(test)]
mod tests {
    // TODO: 如何引入父作用域中的私有/公有定义？
    // 补全引入语句
    use super::*;

    // TODO: 缺少了什么属性宏，导致 cargo test runner 无法识别此测试？
    // 请补齐属性宏
    #[test]
    fn test_box_equivalence() {
        let box1 = BoxDimension::new(10, 20, 30);
        let box2 = BoxDimension::new(10, 20, 30);

        // 验证两个 BoxDimension 实例是否等价
        assert_eq!(box1, box2);
    }

    // 请补齐属性宏
    #[test]
    fn test_volume_calculation() {
        let my_box = BoxDimension::new(5, 5, 5);
        assert_eq!(my_box.volume(), 125, "体积计算不正确！");
    }
}
