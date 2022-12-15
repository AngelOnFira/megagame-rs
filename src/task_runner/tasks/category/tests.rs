pub mod tests {
    use std::sync::Arc;

    use entity::entities::{category, guild, team};
    use rand::{distributions::Alphanumeric, thread_rng, Rng};
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
    use serenity::{model::prelude::Channel, prelude::Context};

    use crate::{
        db_wrapper::DBWrapper,
        task_runner::tasks::{
            category::{
                CategoryCreateError, CategoryHandler, CategoryTasks, CreateCategoryTasks,
                DeleteCategoryTasks,
            },
            test_helpers::{self, DatabaseStatus, DiscordStatus, TestHelpers},
            DatabaseId, DiscordId, TaskType,
        },
        TEST_GUILD_ID,
    };

    pub async fn test_create_category(
        ctx: Arc<Context>,
        db: DBWrapper,
    ) -> Result<(), CategoryCreateError> {
        let test_helper = TestHelpers::new(ctx, db.clone()).await;

        // Create a test team
        let test_team = test_helper.generate_team().await;

        // // Create a test guild
        // let _test_guild = guild::ActiveModel {
        //     discord_id: Set(TEST_GUILD_ID as i32),
        //     ..Default::default()
        // };

        // Create the create category task
        db.add_task(TaskType::CategoryHandler(CategoryHandler {
            guild_id: DiscordId(345993194322001923),
            task: CategoryTasks::Create(CreateCategoryTasks::TeamCategory {
                team_id: DatabaseId(test_team.id as i32),
            }),
        }))
        .await;

        // Sleep for 2 seconds, then check if the category was created
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Check if the category was created
        if let DiscordStatus::DoesNotExist = test_helper
            .check_discord_status(test_helpers::DiscordConstruct::Category {
                name: test_team.name.clone(),
            })
            .await
        {
            return Err(CategoryCreateError::CategoryNotCreated);
        }

        // Check if the category was saved to the database
        if let DatabaseStatus::DoesNotExist = test_helper
            .check_database_status(test_helpers::DatabaseConstruct::Category {
                name: test_team.name.clone(),
            })
            .await
        {
            return Err(CategoryCreateError::CategoryNotSaved);
        }

        // Create the delete category task
        db.add_task(TaskType::CategoryHandler(CategoryHandler {
            guild_id: DiscordId(345993194322001923),
            task: CategoryTasks::Delete(DeleteCategoryTasks::TeamCategory {
                team_id: DatabaseId(test_team.id as i32),
            }),
        }))
        .await;

        // Sleep for 2 seconds, then check if the category was deleted
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Check if the category was deleted
        if let DiscordStatus::Exists = test_helper
            .check_discord_status(test_helpers::DiscordConstruct::Category {
                name: test_team.name,
            })
            .await
        {
            return Err(CategoryCreateError::CategoryNotDeleted);
        }

        Ok(())
    }
}
