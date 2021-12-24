using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Security.Permissions;
using System.Text;

namespace Profiler
{
    #region internal classes 
    public class ProcessSafeHandle : Microsoft.Win32.SafeHandles.SafeHandleZeroOrMinusOneIsInvalid
    {
        private ProcessSafeHandle() : base(true)
        {
        }

        private ProcessSafeHandle(IntPtr handle, bool ownsHandle) : base(ownsHandle)
        {
            SetHandle(handle);
        }

        [SecurityPermission(SecurityAction.LinkDemand, UnmanagedCode = true)]
        protected override bool ReleaseHandle()
        {
            return NativeMethods.CloseHandle(handle);
        }
    }

    public static class NativeMethods
    {
        private const string Kernel32LibraryName = "kernel32.dll";
        private const string Ole32LibraryName = "ole32.dll";
        private const string ShlwapiLibraryName = "shlwapi.dll";
        private const string ShimLibraryName = "mscoree.dll";

        public const int MAX_PATH = 260;

        [System.Runtime.ConstrainedExecution.ReliabilityContract(System.Runtime.ConstrainedExecution.Consistency.WillNotCorruptState, System.Runtime.ConstrainedExecution.Cer.Success)]
        [DllImport(Kernel32LibraryName)]
        public static extern bool CloseHandle(IntPtr handle);

        [DllImport(ShimLibraryName, CharSet = CharSet.Unicode, PreserveSig = false)]
        public static extern void CLRCreateInstance(ref Guid clsid, ref Guid riid, [MarshalAs(UnmanagedType.Interface)]out object metahostInterface);

        public enum ProcessAccessOptions : int
        {
            ProcessTerminate = 0x0001,
            ProcessCreateThread = 0x0002,
            ProcessSetSessionID = 0x0004,
            ProcessVMOperation = 0x0008,
            ProcessVMRead = 0x0010,
            ProcessVMWrite = 0x0020,
            ProcessDupHandle = 0x0040,
            ProcessCreateProcess = 0x0080,
            ProcessSetQuota = 0x0100,
            ProcessSetInformation = 0x0200,
            ProcessQueryInformation = 0x0400,
            ProcessSuspendResume = 0x0800,
            Synchronize = 0x100000,
        }

        [DllImport(Kernel32LibraryName, PreserveSig = true)]
        public static extern ProcessSafeHandle OpenProcess(Int32 dwDesiredAccess, bool bInheritHandle, Int32 dwProcessId);
    }

    // Wrapper for ICLRMetaHost.  Used to find information about runtimes.
    public sealed class CLRMetaHost
    {
        private ICLRMetaHost m_metaHost;

        public const int MaxVersionStringLength = 26; // 24 + NULL and an extra
        private static readonly Guid clsidCLRMetaHost = new Guid("9280188D-0E8E-4867-B30C-7FA83884E8DE");

        public CLRMetaHost()
        {
            object o;
            Guid ifaceId = typeof(ICLRMetaHost).GUID;
            Guid clsid = clsidCLRMetaHost;
            NativeMethods.CLRCreateInstance(ref clsid, ref ifaceId, out o);
            m_metaHost = (ICLRMetaHost)o;
        }

        public CLRRuntimeInfo GetInstalledRuntimeByVersion(string version)
        {
            IEnumerable<CLRRuntimeInfo> runtimes = EnumerateInstalledRuntimes();

            foreach (CLRRuntimeInfo rti in runtimes)
            {
                if (rti.GetVersionString().ToString().ToLower() == version.ToLower())
                {
                    return rti;
                }
            }

            return null;
        }

        public CLRRuntimeInfo GetLoadedRuntimeByVersion(Int32 processId, string version)
        {
            IEnumerable<CLRRuntimeInfo> runtimes = EnumerateLoadedRuntimes(processId);

            foreach (CLRRuntimeInfo rti in runtimes)
            {
                if (rti.GetVersionString().Equals(version, StringComparison.OrdinalIgnoreCase))
                {
                    return rti;
                }
            }

            return null;
        }

        // Retrieve information about runtimes installed on the machine (i.e. in %WINDIR%\Microsoft.NET\)
        public IEnumerable<CLRRuntimeInfo> EnumerateInstalledRuntimes()
        {
            List<CLRRuntimeInfo> runtimes = new List<CLRRuntimeInfo>();
            IEnumUnknown enumRuntimes = m_metaHost.EnumerateInstalledRuntimes();

            // Since we're only getting one at a time, we can pass NULL for count.
            // S_OK also means we got the single element we asked for.
            for (object oIUnknown; enumRuntimes.Next(1, out oIUnknown, IntPtr.Zero) == 0; /* empty */)
            {
                runtimes.Add(new CLRRuntimeInfo(oIUnknown));
            }

            return runtimes;
        }

        // Retrieve information about runtimes that are currently loaded into the target process.
        public IEnumerable<CLRRuntimeInfo> EnumerateLoadedRuntimes(Int32 processId)
        {
            List<CLRRuntimeInfo> runtimes = new List<CLRRuntimeInfo>();
            IEnumUnknown enumRuntimes;

            using (ProcessSafeHandle hProcess = NativeMethods.OpenProcess(
                /*
                (int)(NativeMethods.ProcessAccessOptions.ProcessVMRead |
                                                                        NativeMethods.ProcessAccessOptions.ProcessQueryInformation |
                                                                        NativeMethods.ProcessAccessOptions.ProcessDupHandle |
                                                                        NativeMethods.ProcessAccessOptions.Synchronize),
                 */
                // TODO FIX NOW for debugging. 
                0x1FFFFF, // PROCESS_ALL_ACCESS
                                                                        false, // inherit handle
                                                                        processId))
            {
                if (hProcess.IsInvalid)
                {
                    throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error());
                }

                enumRuntimes = m_metaHost.EnumerateLoadedRuntimes(hProcess);
            }

            // Since we're only getting one at a time, we can pass NULL for count.
            // S_OK also means we got the single element we asked for.
            for (object oIUnknown; enumRuntimes.Next(1, out oIUnknown, IntPtr.Zero) == 0; /* empty */)
            {
                runtimes.Add(new CLRRuntimeInfo(oIUnknown));
            }

            return runtimes;
        }

        public CLRRuntimeInfo GetRuntime(string version)
        {
            Guid ifaceId = typeof(ICLRRuntimeInfo).GUID;
            return new CLRRuntimeInfo(m_metaHost.GetRuntime(version, ref ifaceId));
        }
    }

    // You're expected to get this interface from mscoree!GetCLRMetaHost.
    // Details for APIs are in metahost.idl.
    [ComImport, InterfaceTypeAttribute(ComInterfaceType.InterfaceIsIUnknown), Guid("D332DB9E-B9B3-4125-8207-A14884F53216")]
    internal interface ICLRMetaHost
    {
        [return: MarshalAs(UnmanagedType.Interface)]
        System.Object GetRuntime(
            [In, MarshalAs(UnmanagedType.LPWStr)] string pwzVersion,
            [In] ref Guid riid /*must use typeof(ICLRRuntimeInfo).GUID*/);

        void GetVersionFromFile(
            [In, MarshalAs(UnmanagedType.LPWStr)] string pwzFilePath,
            [Out, MarshalAs(UnmanagedType.LPWStr)] StringBuilder pwzBuffer,
            [In, Out] ref uint pcchBuffer);

        [return: MarshalAs(UnmanagedType.Interface)]
        IEnumUnknown EnumerateInstalledRuntimes();

        [return: MarshalAs(UnmanagedType.Interface)]
        IEnumUnknown EnumerateLoadedRuntimes([In] ProcessSafeHandle hndProcess);
    }

    // Wrapper for ICLRRuntimeInfo.  Represents information about a CLR install instance.
    public sealed class CLRRuntimeInfo
    {
        public CLRRuntimeInfo(System.Object clrRuntimeInfo)
        {
            m_runtimeInfo = (ICLRRuntimeInfo)clrRuntimeInfo;
        }

        public string GetVersionString()
        {
            StringBuilder sb = new StringBuilder(CLRMetaHost.MaxVersionStringLength);
            int verStrLength = sb.Capacity;
            m_runtimeInfo.GetVersionString(sb, ref verStrLength);
            return sb.ToString();
        }
        public string GetRuntimeDirectory()
        {
            StringBuilder sb = new StringBuilder();
            int strLength = 0;
            m_runtimeInfo.GetRuntimeDirectory(sb, ref strLength);
            sb.Capacity = strLength;
            int ret = m_runtimeInfo.GetRuntimeDirectory(sb, ref strLength);
            if (ret < 0)
            {
                Marshal.ThrowExceptionForHR(ret);
            }

            return sb.ToString();
        }

        public ICLRProfiling GetProfilingInterface()
        {
            Guid ifaceId = typeof(ICLRProfiling).GUID;
            Guid clsId = s_ClsIdClrProfiler;
            return (ICLRProfiling)m_runtimeInfo.GetInterface(ref clsId, ref ifaceId);
        }

        public IMetaDataDispenser GetIMetaDataDispenser()
        {
            Guid ifaceId = typeof(ICLRProfiling).GUID;
            Guid clsId = s_ClsIdClrProfiler;
            return (IMetaDataDispenser)m_runtimeInfo.GetInterface(ref clsId, ref ifaceId);
        }

        private static Guid s_ClsIdClrProfiler = new Guid("BD097ED8-733E-43FE-8ED7-A95FF9A8448C");

        private ICLRRuntimeInfo m_runtimeInfo;
    }

    [ComImport, Guid("809C652E-7396-11D2-9771-00A0C9B4D50C"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    public interface IMetaDataDispenser
    {
        /// <summary>
        /// Creates a new area in memory in which you can create new metadata.
        /// </summary>
        /// <param name="rclsid">[in] The CLSID of the version of metadata structures to be created. This value must be CLSID_CorMetaDataRuntime.</param>
        /// <param name="dwCreateFlags">[in] Flags that specify options. This value must be zero.</param>
        /// <param name="riid">
        /// [in] The IID of the desired metadata interface to be returned; the caller will use the interface to create the new metadata.
        /// The value of riid must specify one of the "emit" interfaces. Valid values are IID_IMetaDataEmit, IID_IMetaDataAssemblyEmit, or IID_IMetaDataEmit2. 
        /// </param>
        /// <param name="ppIUnk">[out] The pointer to the returned interface.</param>
        /// <remarks>
        /// STDMETHOD(DefineScope)(         // Return code.
        ///     REFCLSID    rclsid,         // [in] What version to create.
        ///     DWORD       dwCreateFlags,      // [in] Flags on the create.
        ///     REFIID      riid,           // [in] The interface desired.
        ///     IUnknown    **ppIUnk) PURE;     // [out] Return interface on success.
        /// </remarks>
        [PreserveSig]
        void DefineScope(
            [In] ref Guid rclsid,
            [In] uint dwCreateFlags,
            [In] ref Guid riid,
            [Out, MarshalAs(UnmanagedType.Interface)] out object ppIUnk);

        /// <summary>
        /// Opens an existing, on-disk file and maps its metadata into memory.
        /// </summary>
        /// <param name="szScope">[in] The name of the file to be opened. The file must contain common language runtime (CLR) metadata.</param>
        /// <param name="dwOpenFlags">[in] A value of the <c>CorOpenFlags</c> enumeration to specify the mode (read, write, and so on) for opening. </param>
        /// <param name="riid">
        /// [in] The IID of the desired metadata interface to be returned; the caller will use the interface to import (read) or emit (write) metadata. 
        /// The value of riid must specify one of the "import" or "emit" interfaces. Valid values are IID_IMetaDataEmit, IID_IMetaDataImport, IID_IMetaDataAssemblyEmit, IID_IMetaDataAssemblyImport, IID_IMetaDataEmit2, or IID_IMetaDataImport2. 
        /// </param>
        /// <param name="ppIUnk">[out] The pointer to the returned interface.</param>
        /// <remarks>
        /// STDMETHOD(OpenScope)(           // Return code.
        ///     LPCWSTR     szScope,        // [in] The scope to open.
        ///     DWORD       dwOpenFlags,        // [in] Open mode flags.
        ///     REFIID      riid,           // [in] The interface desired.
        ///     IUnknown    **ppIUnk) PURE;     // [out] Return interface on success.
        /// </remarks>
        [PreserveSig]
        void OpenScope(
            [In, MarshalAs(UnmanagedType.LPWStr)] string szScope,
            [In] int dwOpenFlags,
            [In] ref Guid riid,
            [Out, MarshalAs(UnmanagedType.Interface)] out object ppIUnk);

        /// <summary>
        /// Opens an area of memory that contains existing metadata. That is, this method opens a specified area of memory in which the existing data is treated as metadata.
        /// </summary>
        /// <param name="pData">[in] A pointer that specifies the starting address of the memory area.</param>
        /// <param name="cbData">[in] The size of the memory area, in bytes.</param>
        /// <param name="dwOpenFlags">[in] A value of the <c>CorOpenFlags</c> enumeration to specify the mode (read, write, and so on) for opening.</param>
        /// <param name="riid">
        /// [in] The IID of the desired metadata interface to be returned; the caller will use the interface to import (read) or emit (write) metadata. 
        /// The value of riid must specify one of the "import" or "emit" interfaces. Valid values are IID_IMetaDataEmit, IID_IMetaDataImport, IID_IMetaDataAssemblyEmit, IID_IMetaDataAssemblyImport, IID_IMetaDataEmit2, or IID_IMetaDataImport2. 
        /// </param>
        /// <param name="ppIUnk">[out] The pointer to the returned interface.</param>
        /// <remarks>
        /// STDMETHOD(OpenScopeOnMemory)(       // Return code.
        ///     LPCVOID     pData,          // [in] Location of scope data.
        ///     ULONG       cbData,         // [in] Size of the data pointed to by pData.
        ///     DWORD       dwOpenFlags,        // [in] Open mode flags.
        ///     REFIID      riid,           // [in] The interface desired.
        ///     IUnknown    **ppIUnk) PURE;     // [out] Return interface on success.
        /// </remarks>
        [PreserveSig]
        void OpenScopeOnMemory(
            [In] IntPtr pData,
            [In] uint cbData,
            [In] int dwOpenFlags,
            [In] ref Guid riid,
            [Out, MarshalAs(UnmanagedType.IUnknown)] out object ppIUnk);
    }

    // Details about this interface are in metahost.idl.
    [ComImport, InterfaceTypeAttribute(ComInterfaceType.InterfaceIsIUnknown), Guid("BD39D1D2-BA2F-486A-89B0-B4B0CB466891")]
    internal interface ICLRRuntimeInfo
    {
        // Marshalling pcchBuffer as int even though it's unsigned. Max version string is 24 characters, so we should not need to go over 2 billion soon.
        void GetVersionString([Out, MarshalAs(UnmanagedType.LPWStr)] StringBuilder pwzBuffer,
                              [In, Out, MarshalAs(UnmanagedType.U4)] ref int pcchBuffer);

        // Marshalling pcchBuffer as int even though it's unsigned. MAX_PATH is 260, unicode paths are 65535, so we should not need to go over 2 billion soon.
        [PreserveSig]
        int GetRuntimeDirectory([Out, MarshalAs(UnmanagedType.LPWStr)] StringBuilder pwzBuffer,
                                [In, Out, MarshalAs(UnmanagedType.U4)] ref int pcchBuffer);

        int IsLoaded([In] IntPtr hndProcess);

        // Marshal pcchBuffer as int even though it's unsigned. Error strings approaching 2 billion characters are currently unheard-of.
        [LCIDConversion(3)]
        void LoadErrorString([In, MarshalAs(UnmanagedType.U4)] int iResourceID,
                             [Out, MarshalAs(UnmanagedType.LPWStr)] StringBuilder pwzBuffer,
                             [In, Out, MarshalAs(UnmanagedType.U4)] ref int pcchBuffer,
                             [In] int iLocaleID);

        IntPtr LoadLibrary([In, MarshalAs(UnmanagedType.LPWStr)] string pwzDllName);

        IntPtr GetProcAddress([In, MarshalAs(UnmanagedType.LPStr)] string pszProcName);

        [return: MarshalAs(UnmanagedType.IUnknown)]
        System.Object GetInterface([In] ref Guid rclsid, [In] ref Guid riid);

    }

    // Wrapper for standard COM IEnumUnknown, needed for ICLRMetaHost enumeration APIs.
    [ComImport, InterfaceTypeAttribute(ComInterfaceType.InterfaceIsIUnknown), Guid("00000100-0000-0000-C000-000000000046")]
    internal interface IEnumUnknown
    {
        [PreserveSig]
        int Next(
            [In, MarshalAs(UnmanagedType.U4)]
             int celt,
            [Out, MarshalAs(UnmanagedType.IUnknown)]
            out System.Object rgelt,
            IntPtr pceltFetched);

        [PreserveSig]
        int Skip([In, MarshalAs(UnmanagedType.U4)] int celt);

        void Reset();

        void Clone([Out] out IEnumUnknown ppenum);
    }

    /// <summary>
    /// Represents a version of the CLR runtime
    /// </summary>
    public struct ClrDebuggingVersion
    {
        public short StructVersion;
        public short Major;
        public short Minor;
        public short Build;
        public short Revision;
    }

    #endregion
}