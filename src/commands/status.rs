use super::*;

/// Show information about the current project
#[derive(Parser)]
pub struct Args {}

pub async fn command(args: Args) -> Result<()> {
    let configs = Configs::new()?;
    let client = GQLClient::new_authorized(&configs)?;
    let linked_project = configs.get_linked_project()?;

    let vars = queries::project::Variables {
        id: linked_project.project.to_owned(),
    };

    let res = post_graphql::<queries::Project, _>(
        &client,
        "https://backboard.railway.app/graphql/v2",
        vars,
    )
    .await?;

    let body = res.data.context("Failed to retrieve response body")?;

    println!("Project: {}", body.project.name.purple().bold());
    println!(
        "Environment: {}",
        body.project
            .environments
            .edges
            .iter()
            .map(|env| &env.node)
            .find(|env| env.id == linked_project.environment)
            .context("Environment not found!")?
            .name
            .blue()
            .bold()
    );
    println!("Plugins:");
    for plugin in body
        .project
        .plugins
        .edges
        .iter()
        .map(|plugin| &plugin.node)
        .into_iter()
    {
        println!("{}", format!("{:?}", plugin.name).dimmed().bold());
    }

    println!("Services:");
    for service in body
        .project
        .services
        .edges
        .iter()
        .map(|service| &service.node)
        .into_iter()
    {
        println!("{}", service.name.dimmed().bold());
    }
    Ok(())
}