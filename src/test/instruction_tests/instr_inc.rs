use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn inc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[66], OperandSize::Word)
}

#[test]
fn inc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 68], OperandSize::Dword)
}

#[test]
fn inc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 67], OperandSize::Word)
}

#[test]
fn inc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[65], OperandSize::Dword)
}

#[test]
fn inc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 193], OperandSize::Word)
}

#[test]
fn inc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(BX, 5189, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 135, 69, 20], OperandSize::Word)
}

#[test]
fn inc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 193], OperandSize::Dword)
}

#[test]
fn inc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1716018741, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 4, 69, 53, 94, 72, 102], OperandSize::Dword)
}

#[test]
fn inc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 193], OperandSize::Qword)
}

#[test]
fn inc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 366931006, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 132, 120, 62, 236, 222, 21], OperandSize::Qword)
}

#[test]
fn inc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 193], OperandSize::Qword)
}

#[test]
fn inc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 553850230, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 132, 191, 118, 21, 3, 33], OperandSize::Qword)
}

#[test]
fn inc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[68], OperandSize::Word)
}

#[test]
fn inc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 4], OperandSize::Word)
}

#[test]
fn inc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 69], OperandSize::Dword)
}

#[test]
fn inc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(EDI, 843417212, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 135, 124, 134, 69, 50], OperandSize::Dword)
}

#[test]
fn inc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 198], OperandSize::Qword)
}

#[test]
fn inc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 498480708, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 4, 245, 68, 54, 182, 29], OperandSize::Qword)
}

#[test]
fn inc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 65], OperandSize::Word)
}

#[test]
fn inc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(SI, 122, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 68, 122], OperandSize::Word)
}

#[test]
fn inc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[66], OperandSize::Dword)
}

#[test]
fn inc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1432633342, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 132, 67, 254, 63, 100, 85], OperandSize::Dword)
}

#[test]
fn inc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 194], OperandSize::Qword)
}

#[test]
fn inc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 1], OperandSize::Qword)
}

#[test]
fn inc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(RSI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 198], OperandSize::Qword)
}

#[test]
fn inc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 6], OperandSize::Qword)
}

