## Implementation Plan
- We gonna be implementing messaging queues for tap to target communication  (Nats)
- The queue structure will be subject (ETl.data.<stream_id>.<table_name>) and the structure gonna be
```json
{
  "id": "ulid",
  "source": "tap_postgres",
  "stream_name": "users",
  "schema_version": "1.0.2",
  "created_at": "2026-03-08T12:05:00Z",
  "data": [
    { "user_id": 42, "email": "dev@example.com", "action": "update" }
  ],
  "state": {
  },
  "trace_id": "uuid-backbone-trace-123"
}
```


## DB Model
```
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
    stream = models.ForeignKey(Streams, on_delete=models.CASCADE, related_name="configured_tables")
    catalog_item = models.ForeignKey(Catalog, on_delete=models.CASCADE)
    
    # Moved from Catalog to here
    is_selected = models.BooleanField(default=True) 
    replication_method = models.CharField(max_length=50, default="FULL_TABLE")
    replication_key = models.CharField(max_length=255, null=True, blank=True)
    
    class Meta:
        db_table = "stream_configuration"
        unique_together = ('stream', 'catalog_item')


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
        StreamConfiguration, 
        on_delete=models.CASCADE,
        related_name="state"
    )
    
    bookmark_value = models.JSONField(null=True, blank=True) # JSON is better for multi-part keys
    
    bookmark_column = models.CharField(max_length=255, null=True, blank=True)
    bookmark_value = models.CharField(max_length=500, null=True, blank=True)
    bookmark_type = models.CharField(max_length=50, null=True, blank=True)
    records_synced = models.BigIntegerField(default=0)
    last_sync_at = models.DateTimeField(null=True, blank=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        db_table = "state"

```

## Workflow
- Create a connector's that will be a create connector via migration for now later may provide a service for that (Good to have). 
- Create a catalog as well for the respective connector with table and it's respective metadata via query execution now later we can take care.
- On triggering the Stream with tap and target config via cli. The entry has to be created in stream table.
- Subject: ETl.data.<stream_id>.<table_name>
- Based on the config the Stream Configuration needs to be updated with whatever the table's that are provided with the source_config something like this
```json
{
  "connector_config": "",
  "max_results": 10000,
  "max_parallel_requests": 5,
  "batch_size": 50000,
  "tables": {
    "GLTran": {
      "orderby_columns": [
        "LastModifiedDateTime"
      ],
      "filter_conditions": "",
      "key_properties": [
        "BatchNbr","LineNbr","Module"
      ],
      "replication_method": "INCREMENTAL",
      "valid_replication_keys": "LastModifiedDateTime"
    }
  }
}
```
- Create and run stream where tap is a publisher and target is the consumer.
- Sync the state of the stream after each write in the stream table.

- One major thing while state syncing 
- Problem: 
  - Since there can be api connectors and we may might need to use event loop to get the data with high througput but the async does not ensure the order of the data that it will return there is a high chance of integrity problem may happen kinda race condition
- Solution 
  - Approach 1: We can create some sort of batch_write for tap to group the emit the data at once (not memory efficient we need to rethink this case)

## Implementation
Major Fuction will be served in core package that will serve as the parent package for tap and targets and they will inherit the functions from that package to resolve the problem.
- Major functions needed in core package
  - Write_stream in tap where it will get the data and state and it will publish into nats.
  - Listen_stream in target where it will fetch what tap it will need to listen from cli and it will the data consume via a consumer and write in target source and update the state eventually.

