use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 246], OperandSize::Dword)
}

#[test]
fn paddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 40941396, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 12, 133, 84, 183, 112, 2], OperandSize::Dword)
}

#[test]
fn paddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 234], OperandSize::Qword)
}

#[test]
fn paddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM5)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 42], OperandSize::Qword)
}

#[test]
fn paddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 229], OperandSize::Dword)
}

#[test]
fn paddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EAX, 211993008, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 160, 176, 193, 162, 12], OperandSize::Dword)
}

#[test]
fn paddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 197], OperandSize::Qword)
}

#[test]
fn paddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 210472926, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 172, 209, 222, 143, 139, 12], OperandSize::Qword)
}

