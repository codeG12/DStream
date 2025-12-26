from django.db import migrations, models


class Migration(migrations.Migration):
    dependencies = [("Dstream", "0001_initial")]

    operations = [
        migrations.CreateModel(
            name="Connector",
            fields=[
                (
                    "connector_id",
                    models.BigAutoField(primary_key=True, serialize=False),
                ),
                ("connector_name", models.CharField(max_length=255, unique=True)),
                ("connector_type", models.CharField(max_length=20)),
                ("config", models.JSONField()),
                ("is_active", models.BooleanField(default=True)),
                ("created_at", models.DateTimeField(auto_now_add=True)),
                ("updated_at", models.DateTimeField(auto_now=True)),
            ],
            options={
                "db_table": "connectors",
            },
        )
    ]
