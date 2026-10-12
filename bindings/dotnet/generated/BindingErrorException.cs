using System;

namespace Fatoora;

/// <summary>
/// If <c>BindingError</c> is an opaque error that borrows from an opaque
/// parameter or the receiver, that source handle is held by <c>Inner</c>'s
/// managed lifetime edge rather than by this exception class.
/// </summary>
public class BindingErrorException : Exception
{
    public BindingError Inner { get; }

    public BindingErrorException(BindingError inner) : base(
        inner.Message()
    )
    {
        Inner = inner;
    }
}