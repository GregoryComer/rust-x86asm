use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 223], OperandSize::Dword)
}

#[test]
fn phaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 52, 136], OperandSize::Dword)
}

#[test]
fn phaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 202], OperandSize::Qword)
}

#[test]
fn phaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(RBX, 1045374341, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 171, 133, 37, 79, 62], OperandSize::Qword)
}

#[test]
fn phaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 238], OperandSize::Dword)
}

#[test]
fn phaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 19], OperandSize::Dword)
}

#[test]
fn phaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 225], OperandSize::Qword)
}

#[test]
fn phaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RAX, 286995761, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 160, 49, 53, 27, 17], OperandSize::Qword)
}

