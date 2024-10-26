-- Add migration script here
-- if chat changed, notify with chat data
CREATE OR REPLACE FUNCTION add_to_chat() RETURNS TRIGGER AS $$ BEGIN Raise Notice 'add_to_chat: %',
    NEW;
PERFORM pg_notify(
    'chat_updated',
    json_build_object(
        'op',
        TG_OP,
        'old',
        OLD,
        'new',
        NEW
    )::text
);
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER add_to_chat_trigger
AFTER
INSERT
    or
update
    or delete ON chats FOR EACH ROW EXECUTE FUNCTION add_to_chat();
--if new message added,notify with message data
CREATE OR REPLACE FUNCTION add_to_message() RETURNS TRIGGER AS $$ BEGIN if TG_OP = 'INSERT' then Raise Notice 'add_to_message: %',
    NEW;
PERFORM pg_notify(
    'chat_message_created',
    row_to_json(NEW)::text
);
end if;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER add_to_message_trigger
AFTER
INSERT on messages FOR EACH ROW EXECUTE FUNCTION add_to_message();
-- Add migration script here
