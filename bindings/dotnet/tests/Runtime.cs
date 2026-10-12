using System;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using Fatoora.Diplomat;

static class RuntimeTests
{
    static void Check(bool condition, string message) { if (!condition) throw new Exception(message); }
    static unsafe void Main()
    {
        int freed = 0;
        byte* ptr = (byte*)NativeMemory.Alloc(1);
        *ptr = 42;
        var owner = RustHandle<byte>.Owned(ptr, p => { Interlocked.Increment(ref freed); NativeMemory.Free(p); });
        BorrowLease<byte>? lease = owner.Lease(BorrowKind.Shared);
        using var span = new DiplomatBorrowedSpan<byte>(ptr, 1, new[] { LifetimeEdge.Move(ref lease) });
        using var entered = new ManualResetEventSlim();
        using var finish = new ManualResetEventSlim();
        var reader = Task.Run(() => span.WithSpan(s => {
            entered.Set();
            Check(finish.Wait(TimeSpan.FromSeconds(10)), "Disposal thread timed out");
            Check(Volatile.Read(ref freed) == 0, "Source freed during read");
            Check(s[0] == 42, "Read changed");
        }));
        Check(entered.Wait(TimeSpan.FromSeconds(10)), "Reader timed out");
        try
        {
            owner.ReleaseWrapper();
            Check(Volatile.Read(ref freed) == 0, "Dispose bypassed active SafeHandle claim");
            try { span.Dispose(); throw new Exception("Span disposal accepted during read"); }
            catch (InvalidOperationException) { }
        }
        finally { finish.Set(); }
        reader.GetAwaiter().GetResult();
        Check(freed == 1, "Source not freed exactly once after read");
        span.Dispose();
        try { span.Clone(); throw new Exception("Access accepted after dispose"); }
        catch (ObjectDisposedException) { }
        // Race span cleanup against copying; a successful copy must retain its source.
        for (int i = 0; i < 500; i++)
        {
            byte* memory = (byte*)NativeMemory.Alloc(1); *memory = 7;
            var source = RustHandle<byte>.Owned(memory, p => NativeMemory.Free(p));
            BorrowLease<byte>? borrow = source.Lease(BorrowKind.Shared);
            using var view = new DiplomatBorrowedSpan<byte>(memory, 1, new[] { LifetimeEdge.Move(ref borrow) });
            using var start = new Barrier(2);
            var copy = Task.Run(() => {
                start.SignalAndWait();
                try { Check(view.Clone()[0] == 7, "Concurrent copy corrupted"); }
                catch (ObjectDisposedException) { }
            });
            start.SignalAndWait();
            try { view.Dispose(); } catch (InvalidOperationException) { }
            copy.GetAwaiter().GetResult();
            source.ReleaseWrapper();
        }
        Console.WriteLine("Borrowed-span lifetime and concurrency regressions passed");
    }
}
