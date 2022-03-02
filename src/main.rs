use clap::Parser;
use curl::easy::{Easy, List};
use isstran::{args::Args, gitlab::types};
use std::io::stdin;

#[macro_use]
extern crate colour;

type GResult<T> = Result<T, Box<dyn std::error::Error>>;

fn _debug_slice(slice: &[u8], point: usize) {
    if slice.len() > point + 20 {
        red_ln!(
            "{}",
            slice[point - 10..point + 10]
                .iter()
                .map(|b| *b as char)
                .collect::<String>()
        );
    }
}

fn route(url: String, path: String) -> String {
    let mut url = if url.ends_with('/') {
        unsafe { url.get_unchecked(0..url.len() - 1).to_string() }
    } else {
        url
    };
    url.push_str(format!("/api/v4/{}", path).as_str());
    url
}

fn sel_bool_str(source: bool) -> String {
    if source {
        "source".to_string()
    } else {
        "destination".to_string()
    }
}

fn gen_header(args: &Args, source: bool) -> GResult<List> {
    let mut h_list = List::new();
    h_list.append(
        format!(
            "PRIVATE-TOKEN: {}",
            if source {
                args.source_token.to_string()
            } else {
                args.dest_token.to_string()
            }
        )
        .as_str(),
    )?;
    Ok(h_list)
}

fn get_proper_url(args: &Args, source: bool) -> String {
    if source {
        args.source.to_string()
    } else {
        args.dest.to_string()
    }
}

fn retrieve_user_id(args: &Args, source: bool) -> GResult<i64> {
    let mut easy = Easy::new();
    let h_list = gen_header(args, source)?;
    easy.url(
        route(
            get_proper_url(args, source),
            format!(
                "users?username={}",
                if source {
                    args.assignee.clone()
                } else {
                    if let Some(dest_user) = args.dest_user.clone() {
                        dest_user
                    } else {
                        args.assignee.clone()
                    }
                }
            ),
        )
        .as_str(),
    )?;
    let mut assignee_id = 0;
    easy.http_headers(h_list)?;
    let mut buffer = Vec::new();
    {
        let mut trans = easy.transfer();

        trans.write_function(|data| {
            buffer.extend_from_slice(data);
            Ok(data.len())
        })?;
        trans.perform()?;
    }
    if buffer.len() > 2 {
        let assignees: Vec<types::User> = serde_json::from_slice(buffer.as_slice())?;
        assignee_id = assignees[0].id;
        let name = assignees[0].name.clone();
        dark_green_ln!(
            "User Id found in {} for {} is {} ({})",
            sel_bool_str(source),
            args.assignee,
            assignee_id,
            name,
        );
    }
    Ok(assignee_id)
}

fn retrieve_projects(args: &Args, source: bool) -> GResult<Vec<types::SimpleProject>> {
    let mut easy = Easy::new();
    let mut buffer = Vec::new();
    let h_list = gen_header(args, source)?;
    easy.url(
        route(
            get_proper_url(args, source),
            "projects?membership=true&simple=true".to_string(),
        )
        .as_str(),
    )?;
    easy.http_headers(h_list)?;
    {
        let mut trans = easy.transfer();
        trans.write_function(|data| {
            buffer.extend_from_slice(data);
            Ok(data.len())
        })?;
        trans.perform()?;
    }

    let ps: Vec<types::SimpleProject> = serde_json::from_slice(buffer.as_slice()).unwrap();
    dark_green_ln!(
        "Retrieved {} projects from {}",
        ps.len(),
        sel_bool_str(source)
    );
    Ok(ps)
}

fn retrieve_user_issues(
    user_id: i64,
    project_id: i64,
    args: &Args,
    source: bool,
) -> GResult<Vec<types::issue::Issue>> {
    let mut easy = Easy::new();
    let mut buffer = Vec::new();
    let mut is = Vec::new();
    let mut page = 1;
    'pagin: loop {
        let h_list = gen_header(args, source)?;
        easy.url(
            route(
                get_proper_url(args, source),
                format!(
                    "projects/{}/issues?assignee_id={}&scope=all&simple=true&page={}",
                    project_id, user_id, page,
                ),
            )
            .as_str(),
        )?;
        easy.http_headers(h_list)?;
        {
            let mut trans = easy.transfer();
            trans.write_function(|data| {
                buffer.extend_from_slice(data);
                Ok(data.len())
            })?;
            trans.perform()?;
        }

        if buffer.len() <= 2 {
            buffer.clear();
            break 'pagin;
        }
        let mut ret: Vec<types::issue::Issue> = serde_json::from_slice(buffer.as_slice())?;
        is.append(&mut ret);
        buffer.clear();
        page += 1;
    }
    Ok(is)
}

fn retrieve_all_issues(
    project_id: i64,
    args: &Args,
    source: bool,
) -> GResult<Vec<types::issue::Issue>> {
    let mut easy = Easy::new();
    let mut buffer = Vec::new();
    let mut is = Vec::new();
    let mut page = 1;
    'pagin: loop {
        let h_list = gen_header(args, source)?;
        easy.url(
            route(
                get_proper_url(args, source),
                format!(
                    "projects/{}/issues?scope=all&simple=true&page={}",
                    project_id, page,
                ),
            )
            .as_str(),
        )?;
        easy.http_headers(h_list)?;
        {
            let mut trans = easy.transfer();
            trans.write_function(|data| {
                buffer.extend_from_slice(data);
                Ok(data.len())
            })?;
            trans.perform()?;
        }

        if buffer.len() <= 2 {
            buffer.clear();
            break 'pagin;
        }
        let mut ret: Vec<types::issue::Issue> = serde_json::from_slice(buffer.as_slice())?;
        is.append(&mut ret);
        buffer.clear();
        page += 1;
    }
    Ok(is)
}

fn scrap_issues(
    projects: Vec<types::SimpleProject>,
    user_id: Option<i64>,
    args: &Args,
    source: bool,
) -> GResult<Vec<(i64, types::issue::Issue)>> {
    let mut is = Vec::new();
    for p in projects {
        if let Some(uid) = user_id {
            let mut ret = retrieve_user_issues(uid, p.id, args, source)?
                .iter()
                .map(|i| (p.id, i.clone()))
                .collect();
            is.append(&mut ret);
        } else {
            let mut ret = retrieve_all_issues(p.id, args, source)?
                .iter()
                .map(|i| (p.id, i.clone()))
                .collect();
            is.append(&mut ret);
        }
    }
    dark_green_ln!(
        "Total retrieved issues from {}: {}",
        sel_bool_str(source),
        is.len()
    );
    Ok(is)
}

fn update_assignee(issue_iid: i64, user_id: i64, project_id: i64, args: &Args) -> GResult<()> {
    let mut easy = Easy::new();
    let h_list = gen_header(args, false)?;

    easy.url(
        route(
            get_proper_url(args, false),
            format!(
                "projects/{}/issues/{}?assignee_ids={}",
                project_id, issue_iid, user_id
            ),
        )
        .as_str(),
    )?;
    easy.http_headers(h_list)?;
    easy.put(true)?;
    easy.perform()?;

    Ok(())
}

fn main() -> GResult<()> {
    let args = Args::parse();
    let assignee_id = retrieve_user_id(&args, true)?;
    let projects = retrieve_projects(&args, true)?;
    let sourced_issues = scrap_issues(projects, Some(assignee_id), &args, true)?;
    let dprojects = retrieve_projects(&args, false)?;
    let dest_issues = scrap_issues(dprojects, None, &args, false)?;
    let dest_assignee_id = retrieve_user_id(&args, false)?;
    for issue in dest_issues {
        if let Some(found) = sourced_issues.iter().find(|(_p, i)| {
            let i = i.clone();
            let is = issue.1.clone();
            if let Some(refs) = i.references {
                if let Some(irefs) = is.references {
                    irefs.short == refs.short
                } else {
                    false
                }
            } else {
                false
            }
        }) {
            let (_, found_issue) = found.clone();
            let (dproject_id, dissue) = issue.clone();
            yellow_ln!(
                "Found matching issue {} : {} in source",
                found_issue
                    .references
                    .unwrap()
                    .short
                    .unwrap_or_else(|| "".to_string()),
                found_issue.title
            );
            yellow_ln!(
                "Which matches issue {} : {} in destination",
                dissue
                    .references
                    .unwrap()
                    .short
                    .unwrap_or_else(|| "".to_string()),
                dissue.title
            );
            yellow_ln!("Update assignee ? (y/n)");
            if args.yes {
                yellow_ln!("yes");
            } else {
                let mut buffer = String::new();
                stdin().read_line(&mut buffer)?;
                buffer = buffer.to_lowercase();
                buffer = buffer.trim().to_string();

                if buffer.eq("y") || buffer.eq("yes") {
                    update_assignee(dissue.iid, dest_assignee_id, dproject_id, &args)?;
                }
            }
        }
    }
    green_ln!("Done!");
    Ok(())
}
