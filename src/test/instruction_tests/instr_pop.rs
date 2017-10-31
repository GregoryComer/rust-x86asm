use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pop_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(ES)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[7], OperandSize::Word)
}

#[test]
fn pop_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(ES)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[7], OperandSize::Dword)
}

#[test]
fn pop_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(SS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[23], OperandSize::Word)
}

#[test]
fn pop_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(SS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[23], OperandSize::Dword)
}

#[test]
fn pop_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(DS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[31], OperandSize::Word)
}

#[test]
fn pop_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(DS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[31], OperandSize::Dword)
}

#[test]
fn pop_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[91], OperandSize::Word)
}

#[test]
fn pop_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 95], OperandSize::Dword)
}

#[test]
fn pop_9() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 94], OperandSize::Qword)
}

#[test]
fn pop_10() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 94], OperandSize::Word)
}

#[test]
fn pop_11() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[95], OperandSize::Dword)
}

#[test]
fn pop_12() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(RDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[95], OperandSize::Qword)
}

#[test]
fn pop_13() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[91], OperandSize::Word)
}

#[test]
fn pop_14() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[143, 7], OperandSize::Word)
}

#[test]
fn pop_15() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 91], OperandSize::Dword)
}

#[test]
fn pop_16() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1434728891, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 143, 4, 125, 187, 57, 132, 85], OperandSize::Dword)
}

#[test]
fn pop_17() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 91], OperandSize::Qword)
}

#[test]
fn pop_18() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(IndirectScaledDisplaced(RBX, Four, 424806741, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 143, 4, 157, 85, 9, 82, 25], OperandSize::Qword)
}

#[test]
fn pop_19() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 90], OperandSize::Word)
}

#[test]
fn pop_20() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 143, 2], OperandSize::Word)
}

#[test]
fn pop_21() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[91], OperandSize::Dword)
}

#[test]
fn pop_22() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(IndirectDisplaced(EDI, 409113444, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[143, 135, 100, 147, 98, 24], OperandSize::Dword)
}

#[test]
fn pop_23() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(RCX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[89], OperandSize::Qword)
}

#[test]
fn pop_24() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 911766449, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[143, 132, 73, 177, 115, 88, 54], OperandSize::Qword)
}

#[test]
fn pop_25() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Word)
}

#[test]
fn pop_26() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Dword)
}

#[test]
fn pop_27() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Qword)
}

#[test]
fn pop_28() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Word)
}

#[test]
fn pop_29() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Dword)
}

#[test]
fn pop_30() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(FS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 161], OperandSize::Qword)
}

#[test]
fn pop_31() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Word)
}

#[test]
fn pop_32() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Dword)
}

#[test]
fn pop_33() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Qword)
}

#[test]
fn pop_34() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Word)
}

#[test]
fn pop_35() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Dword)
}

#[test]
fn pop_36() {
    run_test(&Instruction { mnemonic: Mnemonic::POP, operand1: Some(Direct(GS)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 169], OperandSize::Qword)
}

