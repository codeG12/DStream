from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [("Dstream", "0002_connector")]

    operations = [
        migrations.CreateModel(
            name="Streams",
            fields=[
                ("stream_id", models.BigAutoField(primary_key=True, serialize=False)),
                ("stream_name", models.CharField(max_length=255)),
                ("is_active", models.BooleanField(default=True)),
                (
                    "last_sync_status",
                    models.CharField(max_length=50, null=True, blank=True),
                ),
                ("last_sync_at", models.DateTimeField(null=True, blank=True)),
                ("created_at", models.DateTimeField(auto_now_add=True)),
                ("updated_at", models.DateTimeField(auto_now=True)),
                (
                    "source_connector",
                    models.JSONField(),
                ),
                (
                    "target_connector",
                    models.JSONField(),
                ),
            ],
            options={
                "db_table": "stream",
            },
        ),
    ]
