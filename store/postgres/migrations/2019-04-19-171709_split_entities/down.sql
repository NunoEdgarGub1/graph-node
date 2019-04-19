insert into public.entities(subgraph, entity, id, data, event_source)
select 'subgraphs', entity, id, data, event_source
from subgraphs.entities;

drop schema subgraphs cascade;

drop table sgd_schemas;

drop function if exists subgraph_log_entity_event();
drop function if exists subgraph_revert_block(varchar, varchar);
