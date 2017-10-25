use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sal_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 225, 55], OperandSize::Word)
}

#[test]
fn sal_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 8841, Some(OperandSize::Byte), None)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 161, 137, 34, 34], OperandSize::Word)
}

#[test]
fn sal_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 121], OperandSize::Dword)
}

#[test]
fn sal_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 36, 186, 5], OperandSize::Dword)
}

#[test]
fn sal_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 31], OperandSize::Qword)
}

#[test]
fn sal_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 33, 112], OperandSize::Qword)
}

#[test]
fn sal_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 227, 29], OperandSize::Qword)
}

#[test]
fn sal_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 38, 123], OperandSize::Qword)
}

#[test]
fn sal_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(SP)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 228, 48], OperandSize::Word)
}

#[test]
fn sal_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 134, Some(OperandSize::Word), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 163, 134, 0, 107], OperandSize::Word)
}

#[test]
fn sal_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BX)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 227, 4], OperandSize::Dword)
}

#[test]
fn sal_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1611874200, Some(OperandSize::Word), None)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 164, 152, 152, 63, 19, 96, 75], OperandSize::Dword)
}

#[test]
fn sal_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DX)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 226, 96], OperandSize::Qword)
}

#[test]
fn sal_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 34, 60], OperandSize::Qword)
}

#[test]
fn sal_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 227, 58], OperandSize::Word)
}

#[test]
fn sal_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 235, Some(OperandSize::Dword), None)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 163, 235, 0, 26], OperandSize::Word)
}

#[test]
fn sal_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 230, 82], OperandSize::Dword)
}

#[test]
fn sal_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(ESI, 190583383, Some(OperandSize::Dword), None)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 166, 87, 18, 92, 11, 97], OperandSize::Dword)
}

#[test]
fn sal_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 225, 16], OperandSize::Qword)
}

#[test]
fn sal_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 35, 65], OperandSize::Qword)
}

#[test]
fn sal_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RSI)), operand2: Some(Literal8(42)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 230, 42], OperandSize::Qword)
}

#[test]
fn sal_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 36, 73, 9], OperandSize::Qword)
}

#[test]
fn sal_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 225], OperandSize::Word)
}

#[test]
fn sal_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 39], OperandSize::Word)
}

#[test]
fn sal_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 225], OperandSize::Dword)
}

#[test]
fn sal_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(EDI, 1846088335, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 167, 143, 18, 9, 110], OperandSize::Dword)
}

#[test]
fn sal_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 227], OperandSize::Qword)
}

#[test]
fn sal_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(RSI, Two, 1716369845, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 36, 117, 181, 185, 77, 102], OperandSize::Qword)
}

#[test]
fn sal_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 226], OperandSize::Qword)
}

#[test]
fn sal_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(RAX, Four, 385716224, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 36, 133, 0, 144, 253, 22], OperandSize::Qword)
}

#[test]
fn sal_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(SP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 228], OperandSize::Word)
}

#[test]
fn sal_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 35], OperandSize::Word)
}

#[test]
fn sal_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 227], OperandSize::Dword)
}

#[test]
fn sal_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(ESI, 1332361186, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 166, 226, 55, 106, 79], OperandSize::Dword)
}

#[test]
fn sal_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 225], OperandSize::Qword)
}

#[test]
fn sal_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 34], OperandSize::Qword)
}

#[test]
fn sal_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 225], OperandSize::Word)
}

#[test]
fn sal_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 206, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 163, 206, 0], OperandSize::Word)
}

#[test]
fn sal_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 229], OperandSize::Dword)
}

#[test]
fn sal_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 33], OperandSize::Dword)
}

#[test]
fn sal_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 229], OperandSize::Qword)
}

#[test]
fn sal_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 129658017, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 164, 207, 161, 108, 186, 7], OperandSize::Qword)
}

#[test]
fn sal_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RSP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 228], OperandSize::Qword)
}

#[test]
fn sal_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 34], OperandSize::Qword)
}

#[test]
fn sal_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Word)
}

#[test]
fn sal_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Memory(16168, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 38, 40, 63], OperandSize::Word)
}

#[test]
fn sal_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 227], OperandSize::Dword)
}

#[test]
fn sal_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 186398090, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 164, 89, 138, 53, 28, 11], OperandSize::Dword)
}

#[test]
fn sal_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 225], OperandSize::Qword)
}

#[test]
fn sal_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 36, 152], OperandSize::Qword)
}

#[test]
fn sal_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 226], OperandSize::Qword)
}

#[test]
fn sal_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 33], OperandSize::Qword)
}

#[test]
fn sal_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 228], OperandSize::Word)
}

#[test]
fn sal_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 125, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 98, 125], OperandSize::Word)
}

#[test]
fn sal_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 225], OperandSize::Dword)
}

#[test]
fn sal_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectDisplaced(ESI, 1314282697, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 166, 201, 92, 86, 78], OperandSize::Dword)
}

#[test]
fn sal_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(BX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 227], OperandSize::Qword)
}

#[test]
fn sal_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledDisplaced(RDI, Four, 2003198899, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 36, 189, 179, 99, 102, 119], OperandSize::Qword)
}

#[test]
fn sal_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 225], OperandSize::Word)
}

#[test]
fn sal_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 238, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 160, 238, 0], OperandSize::Word)
}

#[test]
fn sal_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 229], OperandSize::Dword)
}

#[test]
fn sal_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 32], OperandSize::Dword)
}

#[test]
fn sal_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 231], OperandSize::Qword)
}

#[test]
fn sal_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 39], OperandSize::Qword)
}

#[test]
fn sal_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(Direct(RBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 227], OperandSize::Qword)
}

#[test]
fn sal_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SAL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 804868496, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 164, 249, 144, 81, 249, 47], OperandSize::Qword)
}

