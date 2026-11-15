import inspect

import hanconv
import pytest

CONVERSIONS = {
    "s2t": ("编译", "編譯"),
    "t2s": ("編譯", "编译"),
    "s2tw": ("启用", "啟用"),
    "tw2s": ("啟用", "启用"),
    "s2twp": ("缓存", "快取"),
    "tw2sp": ("快取", "缓存"),
    "t2tw": ("羣組", "群組"),
    "tw2t": ("群組", "羣組"),
    "s2hk": ("图像锐化", "圖像鋭化"),
    "hk2s": ("圖像鋭化", "图像锐化"),
    "t2hk": ("說明文件", "説明文件"),
    "hk2t": ("説明文件", "說明文件"),
    "t2jp": ("圖形", "図形"),
    "jp2t": ("図形", "圖形"),
}


@pytest.mark.parametrize(
    ("name", "source", "expected"),
    [(name, *values) for name, values in CONVERSIONS.items()],
)
def test_conversion(name: str, source: str, expected: str) -> None:
    assert getattr(hanconv, name)(source) == expected


@pytest.mark.parametrize("name", CONVERSIONS)
def test_empty_string(name: str) -> None:
    assert getattr(hanconv, name)("") == ""


@pytest.mark.parametrize("name", CONVERSIONS)
def test_rejects_non_string(name: str) -> None:
    with pytest.raises(TypeError):
        getattr(hanconv, name)(123)


def test_accepts_keyword_argument() -> None:
    assert hanconv.s2t(s="编译") == "編譯"


@pytest.mark.parametrize("name", CONVERSIONS)
def test_signature(name: str) -> None:
    assert tuple(inspect.signature(getattr(hanconv, name)).parameters) == ("s",)


def test_module_docstring() -> None:
    assert hanconv.__doc__ == "Convert between Chinese character variants."


@pytest.mark.parametrize("name", CONVERSIONS)
def test_function_docstring(name: str) -> None:
    docstring = getattr(hanconv, name).__doc__

    assert docstring is not None
    assert docstring.startswith("Convert ")
    assert docstring.endswith(".")
