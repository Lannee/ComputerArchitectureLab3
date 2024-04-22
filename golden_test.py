import pytest
import tempfile
import os
import contextlib
import io

@pytest.mark.golden_test("tests/*.yml")
def test_golden(golden):
    with tempfile.TemporaryDirectory() as tmpdirname:
        source = os.path.join(tmpdirname, "source.asm")
        output = os.path.join(tmpdirname, "output.txt")
        target = os.path.join(tmpdirname, "source.json")
        schedule = os.path.join(tmpdirname, "schedule.json")
        
        with open(source, "w", encoding="utf-8") as file:
            file.write(golden["source"])

        with open(schedule, "w", encoding="utf-8") as file:
            file.write(golden["schedule"])

        os.system(f"cd translator && cargo run -- {source}")
        with open(target, "r") as file:
            code = file.read()
            assert code == golden["machine_code"]
            

        os.system(f"cd processor && cargo run -- {target} {schedule} >> {output}")
        with open(output, "r") as file:
            code = file.read()
            assert code == golden["output"]
    
        with open("processor/log.txt", "r", encoding="utf-8") as file:
            log = file.read()
            assert log == golden["out_log"]