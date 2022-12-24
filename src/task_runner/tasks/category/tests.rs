pub mod tests {
    use std::sync::Arc;

    use serenity::prelude::Context;

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

        // Create the create category task
        db.add_await_task(TaskType::CategoryHandler(CategoryHandler {
            guild_id: DiscordId(TEST_GUILD_ID),
            task: CategoryTasks::Create(CreateCategoryTasks::TeamCategory {
                team_id: DatabaseId(test_team.id as i32),
            }),
        }))
        .await;

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
        db.add_await_task(TaskType::CategoryHandler(CategoryHandler {
            guild_id: DiscordId(345993194322001923),
            task: CategoryTasks::Delete(DeleteCategoryTasks::TeamCategory {
                team_id: DatabaseId(test_team.id as i32),
            }),
        }))
        .await;

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
