from django.db import models


class Connector(models.Model):
    connector_id = models.AutoField(primary_key=True)
    connector_name = models.CharField(max_length=255, unique=True)
    connector_type = models.CharField(
        max_length=20,
    )
    config = models.JSONField()
    is_active = models.BooleanField(default=True)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        db_table = "connectors"


class Streams(models.Model):
    stream_id = models.AutoField(primary_key=True)
    stream_name = models.CharField(max_length=255)

    source_config = models.JSONField()

    target_config = models.JSONField()

    is_active = models.BooleanField(default=True)
    last_sync_status = models.CharField(max_length=50, null=True, blank=True)
    last_sync_at = models.DateTimeField(null=True, blank=True)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        db_table = "stream"


class StreamConfiguration(models.Model):
    """
    This table resolves the flaw.
    It defines exactly which catalog items belong to which stream.
    """

    stream = models.ForeignKey(
        Streams, on_delete=models.CASCADE, related_name="configured_tables"
    )
    catalog_item = models.ForeignKey(Catalog, on_delete=models.CASCADE)

    # Moved from Catalog to here
    is_selected = models.BooleanField(default=True)
    replication_method = models.CharField(max_length=50, default="FULL_TABLE")
    replication_key = models.CharField(max_length=255, null=True, blank=True)

    class Meta:
        db_table = "stream_configuration"
        unique_together = ("stream", "catalog_item")


class Catalog(models.Model):
    catalog_id = models.AutoField(primary_key=True)

    connector_id = models.ForeignKey(
        Connector,
        on_delete=models.CASCADE,
        db_column="connector_id",
    )

    table_name = models.CharField(max_length=255)
    schema_name = models.CharField(max_length=255, null=True, blank=True)
    table_schema = models.JSONField()
    key_properties = models.JSONField(null=True, blank=True)
    replication_method = models.CharField(max_length=50, null=True, blank=True)
    replication_key = models.CharField(max_length=255, null=True, blank=True)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        db_table = "catalog"


class State(models.Model):
    id = models.AutoField(primary_key=True)

    stream_config = models.OneToOneField(
        StreamConfiguration, on_delete=models.CASCADE, related_name="state"
    )

    bookmark_value = models.JSONField(
        null=True, blank=True
    )  # JSON is better for multi-part keys

    bookmark_column = models.CharField(max_length=255, null=True, blank=True)
    bookmark_value = models.CharField(max_length=500, null=True, blank=True)
    bookmark_type = models.CharField(max_length=50, null=True, blank=True)
    records_synced = models.BigIntegerField(default=0)
    last_sync_at = models.DateTimeField(null=True, blank=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        db_table = "state"
