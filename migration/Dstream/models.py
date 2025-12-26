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
    is_selected = models.BooleanField(default=False)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        db_table = "catalog"


class State(models.Model):
    id = models.AutoField(primary_key=True)

    stream_id = models.ForeignKey(
        Streams,
        on_delete=models.CASCADE,
        db_column="stream_id",
    )

    table_name = models.CharField(max_length=255)
    bookmark_column = models.CharField(max_length=255, null=True, blank=True)
    bookmark_value = models.CharField(max_length=500, null=True, blank=True)
    bookmark_type = models.CharField(max_length=50, null=True, blank=True)
    records_synced = models.BigIntegerField(default=0)
    last_sync_at = models.DateTimeField(null=True, blank=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        db_table = "state"
