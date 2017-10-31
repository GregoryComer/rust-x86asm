use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmullw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 249], OperandSize::Dword)
}

#[test]
fn pmullw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1361501793, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 20, 85, 97, 222, 38, 81], OperandSize::Dword)
}

#[test]
fn pmullw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 235], OperandSize::Qword)
}

#[test]
fn pmullw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 2101665178, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 148, 186, 154, 221, 68, 125], OperandSize::Qword)
}

#[test]
fn pmullw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 194], OperandSize::Dword)
}

#[test]
fn pmullw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1108734842, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 12, 69, 122, 243, 21, 66], OperandSize::Dword)
}

#[test]
fn pmullw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 233], OperandSize::Qword)
}

#[test]
fn pmullw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 59], OperandSize::Qword)
}

