#[cfg(test)]
use comrak::ComrakOptions;

#[cfg(test)]
use comrak::nodes::AstNode;

#[cfg(test)]
pub fn gfm_options() -> ComrakOptions<'static> {
    let mut options = ComrakOptions::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.parse.smart = true;
    options
}

#[cfg(test)]
pub fn ast_eq<'a>(a: &'a AstNode<'a>, b: &'a AstNode<'a>) -> bool {
    let a_val = &a.data.borrow().value;
    let b_val = &b.data.borrow().value;

    if a_val != b_val {
        return false;
    }

    let mut a_children = a.children();
    let mut b_children = b.children();

    loop {
        match (a_children.next(), b_children.next()) {
            (Some(ac), Some(bc)) => {
                if !ast_eq(ac, bc) {
                    return false;
                }
            }
            (None, None) => break,
            _ => return false,
        }
    }

    true
}
