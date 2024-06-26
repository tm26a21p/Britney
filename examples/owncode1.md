# Improve Template Parsing Logic in `IssueTemplate` Module

### Description

The current implementation of parsing title and body from the template file in the `IssueTemplate::new()` function assumes that "---" is always present as a separator. This might lead to unexpected behavior if the template file doesn't contain this exact separator, or contains it multiple times. The `generate_issue()` method also doesn't handle cases where placeholders like "{title}" and "{body}" are not replaced because they were not found in the input data.

### Expected Behavior

The template parsing logic should be improved to handle cases when "---" is missing or present multiple times in the template file, ensuring that the title and body variables are correctly set. Additionally, a check should be added in `generate_issue()` method to make sure placeholders have been replaced before returning the issue object.