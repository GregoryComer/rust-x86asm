use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 226], OperandSize::Dword)
}

#[test]
fn paddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 0], OperandSize::Dword)
}

#[test]
fn paddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 227], OperandSize::Qword)
}

#[test]
fn paddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 215300641, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 60, 125, 33, 58, 213, 12], OperandSize::Qword)
}

#[test]
fn paddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 242], OperandSize::Dword)
}

#[test]
fn paddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 999936122, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 172, 199, 122, 208, 153, 59], OperandSize::Dword)
}

#[test]
fn paddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 246], OperandSize::Qword)
}

#[test]
fn paddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 44, 136], OperandSize::Qword)
}

