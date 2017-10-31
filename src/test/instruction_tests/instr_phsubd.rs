use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 225], OperandSize::Dword)
}

#[test]
fn phsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 785664735, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 12, 93, 223, 74, 212, 46], OperandSize::Dword)
}

#[test]
fn phsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 231], OperandSize::Qword)
}

#[test]
fn phsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RBX, 1471884842, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 147, 42, 46, 187, 87], OperandSize::Qword)
}

#[test]
fn phsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 200], OperandSize::Dword)
}

#[test]
fn phsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1491526855, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 12, 245, 199, 228, 230, 88], OperandSize::Dword)
}

#[test]
fn phsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 213], OperandSize::Qword)
}

#[test]
fn phsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RAX, 1296895324, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 152, 92, 13, 77, 77], OperandSize::Qword)
}

