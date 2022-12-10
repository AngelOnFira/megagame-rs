pub mod tests {
    use std::sync::Arc;

    use entity::entities::{category, guild, team};
    use rand::{distributions::Alphanumeric, thread_rng, Rng};
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
    use serenity::{model::prelude::Channel, prelude::Context};

    use crate::{
        db_wrapper::DBWrapper,
        task_runner::tasks::{
            category::{CategoryCreateError, CategoryHandler, CategoryTasks, CreateCategoryTasks},
            TaskType,
        },
        TEST_GUILD_ID,
    };

    pub async fn test_create_category(
        ctx: Arc<Context>,
        db: DBWrapper,
    ) -> Result<(), CategoryCreateError> {
        let team_name: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();

        // Add a sample team to the database
        let test_team = team::ActiveModel {
            name: Set(team_name.clone()),
            ..Default::default()
        }
        .insert(&*db)
        .await
        .unwrap();

        // Create a test guild
        let _test_guild = guild::ActiveModel {
            discord_id: Set(TEST_GUILD_ID as i32),
            ..Default::default()
        };

        db.add_task(TaskType::CategoryHandler(CategoryHandler {
            guild_id: 345993194322001923,
            task: CategoryTasks::Create(CreateCategoryTasks::TeamCategory {
                team_id: test_team.id as u64,
            }),
        }))
        .await;

        // Sleep for 2 seconds, then check if the category was created
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Check if the category was created
        if ctx
            .cache
            .guild(TEST_GUILD_ID)
            .unwrap()
            .channels
            .iter()
            .filter(|(_, channel)| {
                if let Channel::Category(category) = channel {
                    category.name == team_name
                } else {
                    false
                }
            })
            .count()
            != 1
        {
            return Err(CategoryCreateError::CategoryNotCreated);
        }

        // Check if the category was saved to the database
        if category::Entity::find()
            .filter(category::Column::Name.eq(team_name.clone()))
            .one(&*db)
            .await
            .unwrap()
            .is_none()
        {
            return Err(CategoryCreateError::CategoryNotInDatabase);
        }

        // Check if the channel name is the team name

        // TODO: Cleanup

        Ok(())
    }
}
