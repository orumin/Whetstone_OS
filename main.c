#include<efi.h>
#include<efilib.h>

# define MEMMAP_SIZE (1024*1024)
UINT8 memmap[MEMMAP_SIZE * sizeof(EFI_MEMORY_DESCRIPTOR)];

// entry point
EFI_STATUS
efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    EFI_STATUS status = EFI_SUCCESS;

    UINTN memmapSize = MEMMAP_SIZE;
    UINTN mapKey, descriptorSize;
    UINT32 descriptorVersion;

    EFI_MEMORY_DESCRIPTOR *memoryDescriptor;

    EFI_INPUT_KEY key;

    InitializeLib(ImageHandle, SystemTable);
    Print(L"Hello, EFI!\n");

// get firmwware memory map
    status = uefi_call_wrapper(BS->GetMemoryMap, 5, &memmapSize, (EFI_MEMORY_DESCRIPTOR*)memmap, &mapKey, &descriptorSize, &descriptorVersion );
    if (EFI_ERROR(status)) {
        Print(L"Could not get memory map %r\n", status);
        for ( ;; )
            ;
    }

    Print(L"memmapSize = 0x%08x; descriptorSize = %d; sizeof = %d\n\n", memmapSize, descriptorSize, sizeof( EFI_MEMORY_DESCRIPTOR ));
    memoryDescriptor = (EFI_MEMORY_DESCRIPTOR*)memmap;

    do {
        status = uefi_call_wrapper(ST->ConIn->ReadKeyStroke, 2, ST->ConIn, &key);
    } while (status == EFI_NOT_READY);

//    for ( int i=0; i<memmapSize; i+=3 ) {
//        for ( int j=0; j<3; ++j ) {
//            Print( L"Memory Type: %d\r\nPhysical Start: %08x\r\nVirtual Start: %08x\r\nNumber of Pages: %d\r\nAttribute: %08x\r\n\r\n",
//                memoryDescriptor->Type, memoryDescriptor->PhysicalStart, memoryDescriptor->VirtualStart, memoryDescriptor->NumberOfPages, memoryDescriptor->Attribute );
//            memoryDescriptor = (EFI_MEMORY_DESCRIPTOR*)((UINT8*)memoryDescriptor + descriptorSize);
//        }
//    }

    do {
        status = uefi_call_wrapper(ST->ConIn->ReadKeyStroke, 2, ST->ConIn, &key);
    } while (status == EFI_NOT_READY);

    return EFI_SUCCESS;

}

