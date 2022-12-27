pub mod tests {
    use std::sync::Arc;

    use entity::entities::category;

    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
    use serenity::{model::prelude::ChannelType, prelude::Context};

    use crate::{
        db_wrapper::DBWrapper,
        task_runner::tasks::{
            category::CategoryCreateError,
            channel::{ChannelCreateData, ChannelHandler, ChannelTasks},
            test_helpers::TestHelpers,
            DiscordId, TaskType,
        },
        TEST_GUILD_ID,
    };

    pub async fn test_create_channel(
        ctx: Arc<Context>,
        db: DBWrapper,
    ) -> Result<(), CategoryCreateError> {
        let test_helper = TestHelpers::new(ctx.clone(), db.clone()).await;

        // Create a test team
        let test_team = test_helper.generate_team().await;

        // Add the task to create the channel
        db.add_await_task(TaskType::ChannelHandler(ChannelHandler {
            guild_id: DiscordId(TEST_GUILD_ID),
            task: ChannelTasks::Create(ChannelCreateData {
                name: test_team.name.clone(),
                category_id: None,
                kind: ChannelType::Text,
            }),
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
                if let ChannelType::Category = channel.kind {
                    channel.name == test_team.name
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
