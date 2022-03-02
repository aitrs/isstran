use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub state: String,
    pub avatar_url: String,
    pub web_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleProject {
    pub id: i64,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub ssh_url_to_repo: Option<String>,
    pub http_url_to_repo: Option<String>,
    pub web_url: Option<String>,
    pub readme_url: Option<String>,
    pub tag_list: Option<Vec<String>>,
    pub topics: Option<Vec<String>>,
    pub name: Option<String>,
    pub name_with_namespace: Option<String>,
    pub path: Option<String>,
    pub path_with_namespace: Option<String>,
    pub created_at: Option<String>,
    pub last_activity_at: Option<String>,
    pub forks_count: i64,
    pub avatar_url: Option<String>,
    pub star_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: i64,
    pub iid: i64,
    pub due_date: Option<String>,
    pub project_id: Option<i64>,
    pub state: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub mod issue {
    use serde::{Deserialize, Serialize};

    pub type Assignee = crate::gitlab::types::User;

    pub type Author = crate::gitlab::types::User;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TimeStats {
        pub time_estimate: i64,
        pub total_time_spent: i64,
        pub human_time_estimate: Option<String>,
        pub human_total_time_spent: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TaskCompletionStatus {
        pub count: i64,
        pub completed_count: i64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Links {
        #[serde(rename = "self")]
        pub sself: Option<String>,
        pub notes: Option<String>,
        pub award_emoji: Option<String>,
        pub project: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct References {
        pub short: Option<String>,
        pub relative: Option<String>,
        pub full: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Issue {
        pub id: i64,
        pub iid: i64,
        pub title: String,
        pub description: Option<String>,
        pub state: String,
        pub created_at: String,
        pub updated_at: Option<String>,
        pub closed_at: Option<String>,
        pub closed_by: Option<crate::gitlab::types::User>,
        pub labels: Option<Vec<String>>,
        pub milestone: Option<crate::gitlab::types::Milestone>,
        pub assignees: Option<Vec<Assignee>>,
        pub author: Option<Author>,
        #[serde(rename = "type")]
        pub ttype: Option<String>,
        pub assignee: Option<Assignee>,
        pub user_notes_count: i64,
        pub merge_requests_count: i64,
        pub upvotes: i64,
        pub downvotes: i64,
        pub due_date: Option<String>,
        pub confidential: bool,
        pub discussion_locked: Option<bool>,
        pub issue_type: Option<String>,
        pub web_url: String,
        pub time_stats: Option<TimeStats>,
        pub task_completion_status: Option<TaskCompletionStatus>,
        pub has_tasks: bool,
        pub _links: Option<Links>,
        pub references: Option<References>,
        pub moved_to_id: Option<i64>,
        pub service_desk_reply_to: Option<String>,
    }
}
