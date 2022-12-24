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
            channel::{ChannelHandler, ChannelTasks},
            test_helpers::TestHelpers,
            DatabaseId, DiscordId, TaskType,
        },
        TEST_GUILD_ID,
    };

    pub async fn test_create_channel(
        ctx: Arc<Context>,
        db: DBWrapper,
    ) -> Result<(), CategoryCreateError> {
        let test_helper = TestHelpers::new(ctx, db.clone()).await;

        // Create a test team
        let test_team = test_helper.generate_team().await;

        // Add the task to create the channel
        db.add_await_task(TaskType::ChannelHandler(ChannelHandler {
            guild_id: DiscordId(TEST_GUILD_ID),
            task: ChannelTasks::Create(
                crate::task_runner::tasks::channel::CreateChannelTasks::TeamChannel {
                    team_id: DatabaseId(test_team.id),
                    channel_id: DatabaseId(test_team.general_channel_id.unwrap()),
                },
            ),
            category_id: todo!(),
        }))
        .await;

        // Check if the category was created
        if ctx
            .cache
            .guild(TEST_GUILD_ID)
            .unwrap()
            .channels
            .iter()
            .filter(|(_, channel)| {
                if let Channel::Category(category) = channel {
                    category.name == test_team.name
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
            .filter(category::Column::Name.eq(test_team.name.clone()))
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
