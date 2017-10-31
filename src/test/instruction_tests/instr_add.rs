use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn add_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 219], OperandSize::Word)
}

#[test]
fn add_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 25], OperandSize::Word)
}

#[test]
fn add_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 219], OperandSize::Dword)
}

#[test]
fn add_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 2042567662, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 28, 205, 238, 27, 191, 121], OperandSize::Dword)
}

#[test]
fn add_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 210], OperandSize::Qword)
}

#[test]
fn add_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 12, 81], OperandSize::Qword)
}

#[test]
fn add_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 203], OperandSize::Qword)
}

#[test]
fn add_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 28, 200], OperandSize::Qword)
}

#[test]
fn add_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 207], OperandSize::Word)
}

#[test]
fn add_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(BP, 111, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 110, 111], OperandSize::Word)
}

#[test]
fn add_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 242], OperandSize::Dword)
}

#[test]
fn add_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(ECX, 635036447, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 161, 31, 227, 217, 37], OperandSize::Dword)
}

#[test]
fn add_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 217], OperandSize::Qword)
}

#[test]
fn add_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 60, 90], OperandSize::Qword)
}

#[test]
fn add_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 209], OperandSize::Word)
}

#[test]
fn add_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(SI, 10534, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 172, 38, 41], OperandSize::Word)
}

#[test]
fn add_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 245], OperandSize::Dword)
}

#[test]
fn add_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 825172624, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 20, 253, 144, 34, 47, 49], OperandSize::Dword)
}

#[test]
fn add_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 254], OperandSize::Qword)
}

#[test]
fn add_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 107894259, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 60, 221, 243, 85, 110, 6], OperandSize::Qword)
}

#[test]
fn add_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 228], OperandSize::Qword)
}

#[test]
fn add_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RDI, 580920809, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 143, 233, 37, 160, 34], OperandSize::Qword)
}

#[test]
fn add_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 201], OperandSize::Word)
}

#[test]
fn add_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 25], OperandSize::Word)
}

#[test]
fn add_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 209], OperandSize::Dword)
}

#[test]
fn add_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 761634927, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 20, 69, 111, 160, 101, 45], OperandSize::Dword)
}

#[test]
fn add_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 219], OperandSize::Qword)
}

#[test]
fn add_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 20, 115], OperandSize::Qword)
}

#[test]
fn add_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[0, 218], OperandSize::Qword)
}

#[test]
fn add_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BL)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[2, 28, 200], OperandSize::Qword)
}

#[test]
fn add_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 244], OperandSize::Word)
}

#[test]
fn add_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(BP, 1, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 102, 1], OperandSize::Word)
}

#[test]
fn add_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 245], OperandSize::Dword)
}

#[test]
fn add_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 349141520, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 20, 133, 16, 122, 207, 20], OperandSize::Dword)
}

#[test]
fn add_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 252], OperandSize::Qword)
}

#[test]
fn add_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(RCX, 758604977, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 153, 177, 100, 55, 45], OperandSize::Qword)
}

#[test]
fn add_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 1, 204], OperandSize::Word)
}

#[test]
fn add_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 6113, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 3, 147, 225, 23], OperandSize::Word)
}

#[test]
fn add_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 243], OperandSize::Dword)
}

#[test]
fn add_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(ESI, 1570517760, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 158, 0, 51, 156, 93], OperandSize::Dword)
}

#[test]
fn add_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[1, 210], OperandSize::Qword)
}

#[test]
fn add_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[3, 52, 120], OperandSize::Qword)
}

#[test]
fn add_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 1, 241], OperandSize::Qword)
}

#[test]
fn add_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 2011263877, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 3, 172, 146, 133, 115, 225, 119], OperandSize::Qword)
}

#[test]
fn add_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 37], OperandSize::Word)
}

#[test]
fn add_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 4], OperandSize::Dword)
}

#[test]
fn add_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AL)), operand2: Some(Literal8(111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[4, 111], OperandSize::Qword)
}

#[test]
fn add_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(10282)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 42, 40], OperandSize::Word)
}

#[test]
fn add_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(12058)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 26, 47], OperandSize::Dword)
}

#[test]
fn add_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(AX)), operand2: Some(Literal16(8633)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 185, 33], OperandSize::Qword)
}

#[test]
fn add_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(2028682042)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 5, 58, 59, 235, 120], OperandSize::Word)
}

#[test]
fn add_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1925319161)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 249, 9, 194, 114], OperandSize::Dword)
}

#[test]
fn add_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1494727454)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[5, 30, 187, 23, 89], OperandSize::Qword)
}

#[test]
fn add_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RAX)), operand2: Some(Literal32(611242688)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 5, 192, 210, 110, 36], OperandSize::Qword)
}

#[test]
fn add_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 194, 39], OperandSize::Word)
}

#[test]
fn add_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 4, 53], OperandSize::Word)
}

#[test]
fn add_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Literal8(80)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 194, 80], OperandSize::Dword)
}

#[test]
fn add_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 3, 41], OperandSize::Dword)
}

#[test]
fn add_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DL)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 194, 92], OperandSize::Qword)
}

#[test]
fn add_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 806751294, Some(OperandSize::Byte), None)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 132, 186, 62, 12, 22, 48, 86], OperandSize::Qword)
}

#[test]
fn add_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CL)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 193, 70], OperandSize::Qword)
}

#[test]
fn add_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 2, 58], OperandSize::Qword)
}

#[test]
fn add_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BP)), operand2: Some(Literal16(19913)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 197, 201, 77], OperandSize::Word)
}

#[test]
fn add_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal16(28827)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 1, 155, 112], OperandSize::Word)
}

#[test]
fn add_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(CX)), operand2: Some(Literal16(25639)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 193, 39, 100], OperandSize::Dword)
}

#[test]
fn add_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(EDI, 1792583460, Some(OperandSize::Word), None)), operand2: Some(Literal16(12150)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 135, 36, 167, 216, 106, 118, 47], OperandSize::Dword)
}

#[test]
fn add_67() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(SP)), operand2: Some(Literal16(31521)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 196, 33, 123], OperandSize::Qword)
}

#[test]
fn add_68() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1606606578, Some(OperandSize::Word), None)), operand2: Some(Literal16(3581)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 132, 202, 242, 222, 194, 95, 253, 13], OperandSize::Qword)
}

#[test]
fn add_69() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1427148527)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 197, 239, 142, 16, 85], OperandSize::Word)
}

#[test]
fn add_70() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(DI, 51, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1897577259)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 69, 51, 43, 187, 26, 113], OperandSize::Word)
}

#[test]
fn add_71() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(Literal32(1988070991)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 198, 79, 142, 127, 118], OperandSize::Dword)
}

#[test]
fn add_72() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(EBX, Four, 1953539515, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1085604111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 4, 157, 187, 165, 112, 116, 15, 1, 181, 64], OperandSize::Dword)
}

#[test]
fn add_73() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ESI)), operand2: Some(Literal32(1259393339)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 198, 59, 209, 16, 75], OperandSize::Qword)
}

#[test]
fn add_74() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal32(954366551)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 4, 144, 87, 122, 226, 56], OperandSize::Qword)
}

#[test]
fn add_75() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RBP)), operand2: Some(Literal32(2094205032)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 197, 104, 8, 211, 124], OperandSize::Qword)
}

#[test]
fn add_76() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1751111250, Some(OperandSize::Qword), None)), operand2: Some(Literal32(768306679)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 4, 253, 82, 214, 95, 104, 247, 109, 203, 45], OperandSize::Qword)
}

#[test]
fn add_77() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(DX)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 194, 9], OperandSize::Word)
}

#[test]
fn add_78() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Memory(14214, Some(OperandSize::Word), None)), operand2: Some(Literal8(118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 6, 134, 55, 118], OperandSize::Word)
}

#[test]
fn add_79() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BX)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 195, 74], OperandSize::Dword)
}

#[test]
fn add_80() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 3, 112], OperandSize::Dword)
}

#[test]
fn add_81() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(BP)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 197, 26], OperandSize::Qword)
}

#[test]
fn add_82() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 6, 50], OperandSize::Qword)
}

#[test]
fn add_83() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ECX)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 193, 120], OperandSize::Word)
}

#[test]
fn add_84() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 2, 14], OperandSize::Word)
}

#[test]
fn add_85() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(EDX)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 194, 44], OperandSize::Dword)
}

#[test]
fn add_86() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 2, 55], OperandSize::Dword)
}

#[test]
fn add_87() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(ECX)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 193, 65], OperandSize::Qword)
}

#[test]
fn add_88() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectDisplaced(RSI, 1815563113, Some(OperandSize::Dword), None)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 134, 105, 75, 55, 108, 5], OperandSize::Qword)
}

#[test]
fn add_89() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(Direct(RBP)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 197, 54], OperandSize::Qword)
}

#[test]
fn add_90() {
    run_test(&Instruction { mnemonic: Mnemonic::ADD, operand1: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Qword), None)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 4, 130, 108], OperandSize::Qword)
}

