use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn inc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[69], OperandSize::Word)
}

#[test]
fn inc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 66], OperandSize::Dword)
}

#[test]
fn inc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 70], OperandSize::Word)
}

#[test]
fn inc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[70], OperandSize::Dword)
}

#[test]
fn inc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 194], OperandSize::Word)
}

#[test]
fn inc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 1], OperandSize::Word)
}

#[test]
fn inc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 194], OperandSize::Dword)
}

#[test]
fn inc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 323694115, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 132, 254, 35, 46, 75, 19], OperandSize::Dword)
}

#[test]
fn inc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 194], OperandSize::Qword)
}

#[test]
fn inc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(RCX, 1254324400, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 129, 176, 120, 195, 74], OperandSize::Qword)
}

#[test]
fn inc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 195], OperandSize::Qword)
}

#[test]
fn inc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(RCX, 1130081628, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 129, 92, 173, 91, 67], OperandSize::Qword)
}

#[test]
fn inc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[67], OperandSize::Word)
}

#[test]
fn inc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 6700, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 128, 44, 26], OperandSize::Word)
}

#[test]
fn inc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 70], OperandSize::Dword)
}

#[test]
fn inc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 2140398308, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 132, 138, 228, 226, 147, 127], OperandSize::Dword)
}

#[test]
fn inc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 195], OperandSize::Qword)
}

#[test]
fn inc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectDisplaced(RBX, 711090725, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 131, 37, 98, 98, 42], OperandSize::Qword)
}

#[test]
fn inc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 70], OperandSize::Word)
}

#[test]
fn inc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 1], OperandSize::Word)
}

#[test]
fn inc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[66], OperandSize::Dword)
}

#[test]
fn inc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(IndirectScaledDisplaced(ECX, Two, 106526375, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 4, 77, 167, 118, 89, 6], OperandSize::Dword)
}

#[test]
fn inc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 195], OperandSize::Qword)
}

#[test]
fn inc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 7], OperandSize::Qword)
}

#[test]
fn inc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Direct(RCX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 193], OperandSize::Qword)
}

#[test]
fn inc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::INC, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 7], OperandSize::Qword)
}

