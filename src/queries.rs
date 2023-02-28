pub struct ProjectQuery {
    full_path: String,
    namespace_projects_cursor: String,
    groups_cursor: String,
    group_projects_cursor: String,
    initial: bool,
}

impl ProjectQuery {
    pub fn new(full_path: &str) -> Self {
        ProjectQuery {
            full_path: full_path.to_string(),
            namespace_projects_cursor: String::new(),
            groups_cursor: String::new(),
            group_projects_cursor: String::new(),
            initial: true,
        }
    }

    pub fn set_namespace_projects_cursor(&mut self, cursor: &str) {
        self.initial = false;
        self.namespace_projects_cursor = cursor.to_string();

    }

    pub fn set_groups_cursor(&mut self, cursor: &str) {
        self.initial = false;
        self.groups_cursor = cursor.to_string();
    }

    // Still need to figure out how to set this correctly
    // pub fn set_group_projects_cursor(&mut self, cursor: &str) {
    //     self.initial = false;
    //     self.group_projects_cursor = cursor.to_string();
    // }

    pub fn has_next_page(&self) -> bool {
        self.initial ||  self.namespace_projects_cursor.len() > 0 || self.groups_cursor.len() > 0 || self.group_projects_cursor.len() > 0
    }

    pub fn to_string(&self) -> String {
        let mut query = "query { ".to_string();

        if self.initial || self.namespace_projects_cursor.len() > 0 {
            query.push_str(&format!(
                r#"namespace(fullPath: "{full_path}") {{ projects(after: "{namespace_projects_cursor}") {{ pageInfo {{ hasNextPage endCursor }} nodes {{ sshUrlToRepo }} }} }} "#,
                full_path = self.full_path,
                namespace_projects_cursor = self.namespace_projects_cursor
            ));
        }

        if self.initial || self.groups_cursor.len() > 0 || self.group_projects_cursor.len() > 0 {
            query.push_str(&format!(
                r#"group(fullPath: "{full_path}") {{ descendantGroups (after: "{groups_cursor}") {{ pageInfo {{ hasNextPage endCursor }} nodes {{ projects(after: "{group_projects_cursor}") {{ pageInfo {{ hasNextPage endCursor }} nodes {{ sshUrlToRepo }} }} }} }} }} "#,
                full_path = self.full_path,
                groups_cursor = self.groups_cursor,
                group_projects_cursor = self.group_projects_cursor,
            ));
        }

        query.push('}');
        query
    }
}
