from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [("Dstream", "0003_stream")]

    operations = [
        migrations.CreateModel(
            name="Catalog",
            fields=[
                ("catalog_id", models.BigAutoField(primary_key=True, serialize=False)),
                ("table_name", models.CharField(max_length=255)),
                ("table_schema", models.JSONField()),
                ("key_properties", models.JSONField(null=True, blank=True)),
                (
                    "replication_method",
                    models.CharField(max_length=50, null=True, blank=True),
                ),
                (
                    "replication_key",
                    models.CharField(max_length=255, null=True, blank=True),
                ),
                ("is_selected", models.BooleanField(default=False)),
                ("created_at", models.DateTimeField(auto_now_add=True)),
                ("updated_at", models.DateTimeField(auto_now=True)),
                (
                    "connector_id",
                    models.ForeignKey(
                        on_delete=models.CASCADE,
                        to="Dstream.connector",
                        to_field="connector_id",
                    ),
                ),
            ],
            options={
                "db_table": "catalog",
            },
        ),
    ]
