use std::fmt::Display;

pub enum Text<'a, T: Display> {
    PlainText {
        content: T,
    },
    Span {
        content: &'a Text<'a, T>,
        /// bolded?
        b: bool,
        /// italicized?
        i: bool,
        /// underlined?
        u: bool,
        /// strikethrough?
        s: bool,
    },
    Link {
        title: &'a Text<'a, T>,
        destination: &'static str,
    },
}

pub enum Element<'a, T: Display> {
    Heading {
        level: u8,
        content: Text<'a, T>,
    },
    Text {
        content: Text<'a, T>,
    },
    BulletList {
        content: Vec<BulletListNode<'a, T>>,
        children: Vec<Element<'a, T>>,
    },
    Divider,
}

pub enum BulletType {
    FilledCircle,
    EmptyCircle,
    FilledSquare,
    EmptySquare,
}

pub struct BulletListNode<'a, T: Display> {
    content: &'a Text<'a, T>,
    bullet_type: BulletType,
}

// #[cfg(test)]
// mod tests {}
