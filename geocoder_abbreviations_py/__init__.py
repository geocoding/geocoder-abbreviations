from typing import List, Dict


def get_abbreviations_data(language: str) -> List[Dict]:
    """
    Reads language-specific token group data
    :param language: ISO 639-1 language code
    :return: List of token group dictionaries. See documentation for geocoder-abbreviations for further details.
    """
    from json import loads
    from pkg_resources import resource_string
    try:
        return loads(resource_string('tokens', f'{language}.json').decode('utf-8'))
    except FileNotFoundError:
        raise ValueError(f"Tokens file not found for language '{language}'")
