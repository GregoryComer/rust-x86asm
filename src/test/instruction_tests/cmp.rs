use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 203], OperandSize::Word)
}

fn cmp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 19], OperandSize::Word)
}

fn cmp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 210], OperandSize::Dword)
}

fn cmp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(EBX, Two, 536577032, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 28, 93, 8, 132, 251, 31], OperandSize::Dword)
}

fn cmp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 219], OperandSize::Qword)
}

fn cmp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(RDX, Four, 2121713826, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 12, 149, 162, 200, 118, 126], OperandSize::Qword)
}

fn cmp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 210], OperandSize::Qword)
}

fn cmp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(RDI, 2086307932, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 159, 92, 136, 90, 124], OperandSize::Qword)
}

fn cmp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 229], OperandSize::Word)
}

fn cmp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 189, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 168, 189, 0], OperandSize::Word)
}

fn cmp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 252], OperandSize::Dword)
}

fn cmp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 54], OperandSize::Dword)
}

fn cmp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 218], OperandSize::Qword)
}

fn cmp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(RSI, 1897335029, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 166, 245, 8, 23, 113], OperandSize::Qword)
}

fn cmp_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 230], OperandSize::Word)
}

fn cmp_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Memory(31959, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 54, 215, 124], OperandSize::Word)
}

fn cmp_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 209], OperandSize::Dword)
}

fn cmp_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 52, 155], OperandSize::Dword)
}

fn cmp_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 229], OperandSize::Qword)
}

fn cmp_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1031883332, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 148, 129, 68, 74, 129, 61], OperandSize::Qword)
}

fn cmp_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 239], OperandSize::Qword)
}

fn cmp_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 212642884, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 188, 79, 68, 172, 172, 12], OperandSize::Qword)
}

fn cmp_23() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 218], OperandSize::Word)
}

fn cmp_24() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 24], OperandSize::Word)
}

fn cmp_25() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 209], OperandSize::Dword)
}

fn cmp_26() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1918228665, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 12, 117, 185, 216, 85, 114], OperandSize::Dword)
}

fn cmp_27() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 202], OperandSize::Qword)
}

fn cmp_28() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1026505262, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 28, 197, 46, 58, 47, 61], OperandSize::Qword)
}

fn cmp_29() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[56, 203], OperandSize::Qword)
}

fn cmp_30() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[58, 12, 138], OperandSize::Qword)
}

fn cmp_31() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 244], OperandSize::Word)
}

fn cmp_32() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(DI, 250, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 141, 250, 0], OperandSize::Word)
}

fn cmp_33() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 251], OperandSize::Dword)
}

fn cmp_34() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DI)), operand2: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 62], OperandSize::Dword)
}

fn cmp_35() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 209], OperandSize::Qword)
}

fn cmp_36() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(RDI, 1857992281, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 183, 89, 182, 190, 110], OperandSize::Qword)
}

fn cmp_37() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 57, 225], OperandSize::Word)
}

fn cmp_38() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(BX, 25860, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 59, 183, 4, 101], OperandSize::Word)
}

fn cmp_39() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 227], OperandSize::Dword)
}

fn cmp_40() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(ECX, 1719819487, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 145, 223, 92, 130, 102], OperandSize::Dword)
}

fn cmp_41() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[57, 243], OperandSize::Qword)
}

fn cmp_42() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[59, 35], OperandSize::Qword)
}

fn cmp_43() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 57, 214], OperandSize::Qword)
}

fn cmp_44() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1825367550, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 59, 164, 177, 254, 229, 204, 108], OperandSize::Qword)
}

fn cmp_45() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 22], OperandSize::Word)
}

fn cmp_46() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 19], OperandSize::Dword)
}

fn cmp_47() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AL)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[60, 81], OperandSize::Qword)
}

fn cmp_48() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(4641)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 33, 18], OperandSize::Word)
}

fn cmp_49() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(10255)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 15, 40], OperandSize::Dword)
}

fn cmp_50() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(AX)), operand2: Some(Literal16(21985)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 225, 85], OperandSize::Qword)
}

fn cmp_51() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1347797765)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 61, 5, 195, 85, 80], OperandSize::Word)
}

fn cmp_52() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(367480829)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 253, 79, 231, 21], OperandSize::Dword)
}

fn cmp_53() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EAX)), operand2: Some(Literal32(2042855317)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[61, 149, 127, 195, 121], OperandSize::Qword)
}

fn cmp_54() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1726395969)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 61, 65, 182, 230, 102], OperandSize::Qword)
}

fn cmp_55() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CL)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 249, 7], OperandSize::Word)
}

fn cmp_56() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 61, 93], OperandSize::Word)
}

fn cmp_57() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 251, 97], OperandSize::Dword)
}

fn cmp_58() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 63, 30], OperandSize::Dword)
}

fn cmp_59() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BL)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 251, 89], OperandSize::Qword)
}

fn cmp_60() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 60, 146, 13], OperandSize::Qword)
}

fn cmp_61() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DL)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 250, 0], OperandSize::Qword)
}

fn cmp_62() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 417226264, Some(OperandSize::Byte), None)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 188, 90, 24, 94, 222, 24, 12], OperandSize::Qword)
}

fn cmp_63() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(Literal16(26880)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 252, 0, 105], OperandSize::Word)
}

fn cmp_64() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(SI, 82, Some(OperandSize::Word), None)), operand2: Some(Literal16(26252)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 124, 82, 140, 102], OperandSize::Word)
}

fn cmp_65() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BP)), operand2: Some(Literal16(32126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 253, 126, 125], OperandSize::Dword)
}

fn cmp_66() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 599382474, Some(OperandSize::Word), None)), operand2: Some(Literal16(15975)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 60, 221, 202, 217, 185, 35, 103, 62], OperandSize::Dword)
}

fn cmp_67() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(DI)), operand2: Some(Literal16(21144)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 255, 152, 82], OperandSize::Qword)
}

fn cmp_68() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 192019599, Some(OperandSize::Word), None)), operand2: Some(Literal16(28502)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 188, 136, 143, 252, 113, 11, 86, 111], OperandSize::Qword)
}

fn cmp_69() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBX)), operand2: Some(Literal32(671851597)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 251, 77, 164, 11, 40], OperandSize::Word)
}

fn cmp_70() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(SI, 23149, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1894081732)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 188, 109, 90, 196, 100, 229, 112], OperandSize::Word)
}

fn cmp_71() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBP)), operand2: Some(Literal32(571979322)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 253, 58, 182, 23, 34], OperandSize::Dword)
}

fn cmp_72() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal32(2007038364)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 60, 214, 156, 249, 160, 119], OperandSize::Dword)
}

fn cmp_73() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EBP)), operand2: Some(Literal32(849025419)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 253, 139, 25, 155, 50], OperandSize::Qword)
}

fn cmp_74() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 166564846, Some(OperandSize::Dword), None)), operand2: Some(Literal32(231809818)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 188, 90, 238, 147, 237, 9, 26, 35, 209, 13], OperandSize::Qword)
}

fn cmp_75() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RDI)), operand2: Some(Literal32(1155005174)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 255, 246, 250, 215, 68], OperandSize::Qword)
}

fn cmp_76() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1533054685, Some(OperandSize::Qword), None)), operand2: Some(Literal32(854433758)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 60, 213, 221, 142, 96, 91, 222, 159, 237, 50], OperandSize::Qword)
}

fn cmp_77() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(SP)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 252, 81], OperandSize::Word)
}

fn cmp_78() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(BX, 9899, Some(OperandSize::Word), None)), operand2: Some(Literal8(10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 191, 171, 38, 10], OperandSize::Word)
}

fn cmp_79() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(BP)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 253, 77], OperandSize::Dword)
}

fn cmp_80() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 281466997, Some(OperandSize::Word), None)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 188, 118, 117, 216, 198, 16, 91], OperandSize::Dword)
}

fn cmp_81() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(CX)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 249, 116], OperandSize::Qword)
}

fn cmp_82() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 59, 39], OperandSize::Qword)
}

fn cmp_83() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ESP)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 252, 44], OperandSize::Word)
}

fn cmp_84() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectDisplaced(BX, 207, Some(OperandSize::Dword), None)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 191, 207, 0, 81], OperandSize::Word)
}

fn cmp_85() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(ECX)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 249, 38], OperandSize::Dword)
}

fn cmp_86() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 4599614, Some(OperandSize::Dword), None)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 188, 80, 62, 47, 70, 0, 105], OperandSize::Dword)
}

fn cmp_87() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(EDI)), operand2: Some(Literal8(91)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 255, 91], OperandSize::Qword)
}

fn cmp_88() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 57, 97], OperandSize::Qword)
}

fn cmp_89() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(Direct(RDX)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 250, 56], OperandSize::Qword)
}

fn cmp_90() {
    run_test(&Instruction { mnemonic: Mnemonic::CMP, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1634964012, Some(OperandSize::Qword), None)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 60, 205, 44, 146, 115, 97, 37], OperandSize::Qword)
}

