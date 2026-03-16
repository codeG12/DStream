from django.db import migrations, models


class Migration(migrations.Migration):
    dependencies = [("Dstream", "0004_catalog")]

    operations = [
        migrations.CreateModel(
            name="State",
            fields=[
                (
                    "id",
                    models.BigAutoField(primary_key=True, serialize=False),
                ),
                (
                    "bookmark_column",
                    models.CharField(max_length=255, null=True, blank=True),
                ),
                (
                    "bookmark_value",
                    models.CharField(max_length=500, null=True, blank=True),
                ),
                ("records_synced", models.BigIntegerField(default=0)),
                ("last_sync_at", models.DateTimeField(null=True, blank=True)),
                ("updated_at", models.DateTimeField(auto_now=True)),
                (
                    "stream_id",
                    models.ForeignKey(
                        on_delete=models.CASCADE,
                        to="Dstream.streams",
                        to_field="stream_id",
                    ),
                ),
                ("table_name", models.CharField(max_length=255)),
            ],
            options={
                "db_table": "state",
            },
        ),
    ]
