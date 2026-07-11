#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitStatusEntry {
    pub index_status: char,
    pub worktree_status: char,
    pub path: String,
    pub original_path: Option<String>,
}

impl GitStatusEntry {
    pub(crate) fn parse(line: &str) -> Option<Self> {
        let mut chars = line.chars();
        let index_status = chars.next()?;
        let worktree_status = chars.next()?;
        if chars.next()? != ' ' {
            return None;
        }
        Some(Self {
            index_status,
            worktree_status,
            path: chars.collect(),
            original_path: None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitBranch {
    pub name: String,
    pub current: bool,
}
