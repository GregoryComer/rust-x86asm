use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn btr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 217], OperandSize::Word)
}

#[test]
fn btr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 14857, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 177, 9, 58], OperandSize::Word)
}

#[test]
fn btr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 209], OperandSize::Dword)
}

#[test]
fn btr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 44, 159], OperandSize::Dword)
}

#[test]
fn btr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 238], OperandSize::Qword)
}

#[test]
fn btr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 56], OperandSize::Qword)
}

#[test]
fn btr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 234], OperandSize::Word)
}

#[test]
fn btr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Memory(1396, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 179, 14, 116, 5], OperandSize::Word)
}

#[test]
fn btr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 223], OperandSize::Dword)
}

#[test]
fn btr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 2129085877, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 28, 221, 181, 69, 231, 126], OperandSize::Dword)
}

#[test]
fn btr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 249], OperandSize::Qword)
}

#[test]
fn btr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectDisplaced(RBX, 160146644, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 179, 139, 212, 164, 139, 9], OperandSize::Qword)
}

#[test]
fn btr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 179, 246], OperandSize::Qword)
}

#[test]
fn btr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 486127090, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 179, 44, 221, 242, 181, 249, 28], OperandSize::Qword)
}

#[test]
fn btr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(CX)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 241, 32], OperandSize::Word)
}

#[test]
fn btr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Memory(21065, Some(OperandSize::Word), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 54, 73, 82, 115], OperandSize::Word)
}

#[test]
fn btr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DX)), operand2: Some(Literal8(36)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 242, 36], OperandSize::Dword)
}

#[test]
fn btr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 55, 6], OperandSize::Dword)
}

#[test]
fn btr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(DX)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 242, 84], OperandSize::Qword)
}

#[test]
fn btr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 54, 6], OperandSize::Qword)
}

#[test]
fn btr_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 244, 7], OperandSize::Word)
}

#[test]
fn btr_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 17535, Some(OperandSize::Dword), None)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 176, 127, 68, 31], OperandSize::Word)
}

#[test]
fn btr_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 247, 100], OperandSize::Dword)
}

#[test]
fn btr_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 1529250623, Some(OperandSize::Dword), None)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 180, 185, 63, 131, 38, 91, 30], OperandSize::Dword)
}

#[test]
fn btr_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(124)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 241, 124], OperandSize::Qword)
}

#[test]
fn btr_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 52, 64, 96], OperandSize::Qword)
}

#[test]
fn btr_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(Direct(RSI)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 246, 78], OperandSize::Qword)
}

#[test]
fn btr_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTR, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 923735471, Some(OperandSize::Qword), None)), operand2: Some(Literal8(42)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 52, 205, 175, 21, 15, 55, 42], OperandSize::Qword)
}

